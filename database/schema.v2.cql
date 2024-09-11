-- Create the openmeet keyspace
CREATE KEYSPACE IF NOT EXISTS openmeet
WITH replication = {
  'class': 'SimpleStrategy',
  'replication_factor': 3
};

USE openmeet;

-- Users table
CREATE TABLE openmeet.users (
  user_id UUID,
  username TEXT,
  email TEXT,
  password_hash TEXT,
  description TEXT,
  interests SET<TEXT>,
  created_at TIMESTAMP,
  updated_at TIMESTAMP,
  last_login TIMESTAMP,
  PRIMARY KEY (user_id)
);
CREATE INDEX ON openmeet.users (email);
CREATE INDEX ON openmeet.users (username);

CREATE TABLE openmeet.email_index (
  email TEXT PRIMARY KEY,
  user_id UUID
);

-- Groups table
CREATE TABLE groups (
  group_id UUID,
  name TEXT,
  description TEXT,
  interests SET<TEXT>,
  created_at TIMESTAMP,
  updated_at TIMESTAMP,
  PRIMARY KEY (group_id)
);
CREATE INDEX ON groups (name);

-- Events table
CREATE TABLE events (
  event_id UUID,
  group_id UUID,
  title TEXT,
  description TEXT,
  start_time TIMESTAMP,
  end_time TIMESTAMP,
  location TEXT,
  lat DOUBLE,
  lon DOUBLE,
  is_public BOOLEAN,
  created_at TIMESTAMP,
  updated_at TIMESTAMP,
  PRIMARY KEY ((group_id), start_time, event_id)
) WITH CLUSTERING ORDER BY (start_time DESC, event_id ASC);

-- Comments table
CREATE TABLE comments (
  event_id UUID,
  comment_id UUID,
  user_id UUID,
  content TEXT,
  created_at TIMESTAMP,
  updated_at TIMESTAMP,
  PRIMARY KEY ((event_id), created_at, comment_id)
) WITH CLUSTERING ORDER BY (created_at DESC, comment_id ASC);

-- User comments table
CREATE TABLE user_comments (
  user_id UUID,
  comment_id UUID,
  event_id UUID,
  content TEXT,
  created_at TIMESTAMP,
  PRIMARY KEY ((user_id), created_at, comment_id)
) WITH CLUSTERING ORDER BY (created_at DESC, comment_id ASC);

-- Events by location table
CREATE TABLE events_by_location (
  location_bucket TEXT,
  event_id UUID,
  group_id UUID,
  title TEXT,
  start_time TIMESTAMP,
  lat DOUBLE,
  lon DOUBLE,
  PRIMARY KEY ((location_bucket), start_time, event_id)
) WITH CLUSTERING ORDER BY (start_time DESC, event_id ASC);

-- Group members table
CREATE TABLE group_members (
  group_id UUID,
  user_id UUID,
  joined_at TIMESTAMP,
  PRIMARY KEY ((group_id), user_id)
);

-- Event attendees table
CREATE TABLE event_attendees (
  event_id UUID,
  user_id UUID,
  rsvp_status TEXT,
  is_host BOOLEAN,
  PRIMARY KEY ((event_id), user_id)
);

-- User events table
CREATE TABLE user_events (
  user_id UUID,
  event_id UUID,
  group_id UUID,
  start_time TIMESTAMP,
  rsvp_status TEXT,
  is_host BOOLEAN,
  PRIMARY KEY ((user_id), start_time, event_id)
) WITH CLUSTERING ORDER BY (start_time DESC, event_id ASC);

-- Users by interest table
CREATE TABLE users_by_interest (
  interest TEXT,
  user_id UUID,
  username TEXT,
  PRIMARY KEY ((interest), user_id)
);

-- Groups by interest table
CREATE TABLE groups_by_interest (
  interest TEXT,
  group_id UUID,
  name TEXT,
  PRIMARY KEY ((interest), group_id)
);