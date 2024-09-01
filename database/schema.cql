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

-- Populate Users table
INSERT INTO users (user_id, username, email, password_hash, created_at, updated_at, last_login)
VALUES (uuid(), 'john_doe', 'john@example.com', 'hashed_password_1', toTimestamp(now()), toTimestamp(now()), toTimestamp(now()));

INSERT INTO users (user_id, username, email, password_hash, created_at, updated_at, last_login)
VALUES (uuid(), 'jane_smith', 'jane@example.com', 'hashed_password_2', toTimestamp(now()), toTimestamp(now()), toTimestamp(now()));

-- Populate Events table
INSERT INTO events (event_id, creator_id, title, description, start_time, end_time, lat, lon, address, created_at, updated_at)
VALUES (uuid(), 123e4567-e89b-12d3-a456-426614174000, 'Tech Meetup', 'Join us for a tech discussion', '2023-06-15 18:00:00', '2023-06-15 20:00:00', 40.7128, -74.0060, '123 Main St, New York, NY', toTimestamp(now()), toTimestamp(now()));

INSERT INTO events (event_id, creator_id, title, description, start_time, end_time, lat, lon, address, created_at, updated_at)
VALUES (uuid(), 123e4567-e89b-12d3-a456-426614174000, 'Yoga in the Park', 'Relaxing yoga session', '2023-06-20 09:00:00', '2023-06-20 10:30:00', 40.7829, -73.9654, 'Central Park, New York, NY', toTimestamp(now()), toTimestamp(now()));

-- Populate Events by location table
INSERT INTO events_by_location (location_bucket, event_id, creator_id, title, start_time, lat, lon)
VALUES ('NYC', 123e4567-e89b-12d3-a456-426614174001, 123e4567-e89b-12d3-a456-426614174000, 'Tech Meetup', '2023-06-15 18:00:00', 40.7128, -74.0060);

INSERT INTO events_by_location (location_bucket, event_id, creator_id, title, start_time, lat, lon)
VALUES ('NYC', 123e4567-e89b-12d3-a456-426614174002, 123e4567-e89b-12d3-a456-426614174000, 'Yoga in the Park', '2023-06-20 09:00:00', 40.7829, -73.9654);

-- Populate Comments table
INSERT INTO comments (event_id, comment_id, user_id, content, created_at, updated_at)
VALUES (123e4567-e89b-12d3-a456-426614174001, uuid(), 123e4567-e89b-12d3-a456-426614174003, 'Looking forward to this meetup!', toTimestamp(now()), toTimestamp(now()));

INSERT INTO comments (event_id, comment_id, user_id, content, created_at, updated_at)
VALUES (123e4567-e89b-12d3-a456-426614174002, uuid(), 123e4567-e89b-12d3-a456-426614174004, 'Is this suitable for beginners?', toTimestamp(now()), toTimestamp(now()));

-- Populate User comments table
INSERT INTO user_comments (user_id, comment_id, event_id, content, created_at)
VALUES (123e4567-e89b-12d3-a456-426614174003, 123e4567-e89b-12d3-a456-426614174005, 123e4567-e89b-12d3-a456-426614174001, 'Looking forward to this meetup!', toTimestamp(now()));

INSERT INTO user_comments (user_id, comment_id, event_id, content, created_at)
VALUES (123e4567-e89b-12d3-a456-426614174004, 123e4567-e89b-12d3-a456-426614174006, 123e4567-e89b-12d3-a456-426614174002, 'Is this suitable for beginners?', toTimestamp(now()));