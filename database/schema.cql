-- Create the openmeet keyspace
CREATE KEYSPACE IF NOT EXISTS openmeet
WITH replication = {
  'class': 'SimpleStrategy',
  'replication_factor': 1
};

-- Use the openmeet keyspace
USE openmeet;

-- Users table
CREATE TABLE openmeet.users (
  user_id UUID,
  username TEXT,
  email TEXT,
  password_hash TEXT,
  created_at TIMESTAMP,
  updated_at TIMESTAMP,
  last_login TIMESTAMP,
  PRIMARY KEY (user_id)
);
CREATE TABLE openmeet.email_index (
  email TEXT PRIMARY KEY,
  user_id UUID
);
CREATE INDEX ON openmeet.users (email);


-- Events table
CREATE TABLE events (
  event_id UUID,
  creator_id UUID,
  title TEXT,
  description TEXT,
  start_time TIMESTAMP,
  end_time TIMESTAMP,
  lat DOUBLE,
  lon DOUBLE,
  address TEXT,
  created_at TIMESTAMP,
  updated_at TIMESTAMP,
  PRIMARY KEY ((creator_id), start_time, event_id)
) WITH CLUSTERING ORDER BY (start_time DESC, event_id ASC);

-- Events by location table (for geospatial queries)
CREATE TABLE events_by_location (
  location_bucket TEXT,
  event_id UUID,
  creator_id UUID,
  title TEXT,
  start_time TIMESTAMP,
  lat DOUBLE,
  lon DOUBLE,
  PRIMARY KEY ((location_bucket), start_time, event_id)
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

-- User comments table (for querying all comments by a user)
CREATE TABLE user_comments (
  user_id UUID,
  comment_id UUID,
  event_id UUID,
  content TEXT,
  created_at TIMESTAMP,
  PRIMARY KEY ((user_id), created_at, comment_id)
) WITH CLUSTERING ORDER BY (created_at DESC, comment_id ASC);

-- data


-- Insert sample data into users table
INSERT INTO openmeet.users (user_id, username, email, password_hash, created_at, updated_at, last_login) VALUES (uuid(), 'john_doe', 'john@example.com', 'hashed_password_1', toTimestamp(now()), toTimestamp(now()), toTimestamp(now()));
INSERT INTO openmeet.users (user_id, username, email, password_hash, created_at, updated_at, last_login) VALUES (uuid(), 'jane_smith', 'jane@example.com', 'hashed_password_2', toTimestamp(now()), toTimestamp(now()), toTimestamp(now()));

-- Insert sample data into email_index table
INSERT INTO openmeet.email_index (email, user_id) VALUES ('john@example.com', uuid());
INSERT INTO openmeet.email_index (email, user_id) VALUES ('jane@example.com', uuid());

-- Insert sample data into events table
INSERT INTO events (event_id, creator_id, title, description, start_time, end_time, lat, lon, address, created_at, updated_at) VALUES (uuid(), uuid(), 'Sample Event 1', 'Description for event 1', toTimestamp(now()), toTimestamp(now() + 3600), 37.7749, -122.4194, '123 Sample St, San Francisco, CA', toTimestamp(now()), toTimestamp(now()));
INSERT INTO events (event_id, creator_id, title, description, start_time, end_time, lat, lon, address, created_at, updated_at) VALUES (uuid(), uuid(), 'Sample Event 2', 'Description for event 2', toTimestamp(now()), toTimestamp(now() + 7200), 34.0522, -118.2437, '456 Sample Ave, Los Angeles, CA', toTimestamp(now()), toTimestamp(now()));

-- Insert sample data into comments table
INSERT INTO comments (event_id, comment_id, user_id, content, created_at, updated_at) VALUES (uuid(), uuid(), uuid(), 'This is a comment for event 1', toTimestamp(now()), toTimestamp(now()));
INSERT INTO comments (event_id, comment_id, user_id, content, created_at, updated_at) VALUES (uuid(), uuid(), uuid(), 'This is a comment for event 2', toTimestamp(now()), toTimestamp(now()));

-- Insert sample data into user_comments table
INSERT INTO user_comments (user_id, comment_id, event_id, content, created_at) VALUES (uuid(), uuid(), uuid(), 'User comment for event 1', toTimestamp(now()));
INSERT INTO user_comments (user_id, comment_id, event_id, content, created_at) VALUES (uuid(), uuid(), uuid(), 'User comment for event 2', toTimestamp(now()));
