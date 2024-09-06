# OpenMeet: Distributed Open-Source Event Platform

## Description
OpenMeet is an open-source, self-hostable event platform that creates a distributed network for organizing and discovering local events and interest groups. Users can run their own instances, which may sync with other instances to form a larger network.  OpenMeet hosts a public instance, and users may connect with that if they do not want to run their own instance. If you want total ownership of your data, you'll need tojoin a community that is running their own instance. Joinging a community should be free, though large groups may charge dues to cover the cost of hosting.

## Importance
This approach promotes data ownership, privacy, and community control while still allowing for a wide-reaching network effect. It provides an alternative to centralized platforms without the complexities of blockchain technology.  The goal of a free public service opens the community to those that are left out of a system which requires $30 per month.

## Demand
There's growing interest in decentralized and self-hosted alternatives to big tech platforms.
Privacy concerns abound.  There are  
The success of platforms like Mastodon demonstrates a market for distributed social technologies.
The events industry remains substantial, indicating potential for adoption.

## Technical Feasibility
This could be implemented as a server application written in a language like Rust, Go, or Node.js for efficiency and ease of deployment. It would use Cassandra for data replication between instances. A REST API would allow for custom frontends, with a default web interface provided.

Key components:
1. Server application for hosting events and user data
2. Cassandra instances for syncing between nodes
3. Federation protocol for instance communication
4. Web interface for users to interact with the platform

## Considerations
- Federation: Ensuring data consistency across instances while allowing for offline operation and conflict resolution.
- Implementing effective discovery mechanisms for events across the platform
- Balancing between instance autonomy and network-wide policies (e.g., for moderation). How do we make sure trolls aren't a problem, or spam tactics? Do we build that into the tool, or is that an add on worth selling to institutions?
- Developing a sustainable model for maintaining and updating the open-source codebase.

## Potential Advantages
- Users and communities can have full control over their data by running their own instances.
- Resilience against central point of failure or control.
- Potential for customization and extension by the community.
- Lower overall costs distributed among instance operators.

## Potential Challenges
- Achieving critical mass of users and instance operators.
- Ensuring ease of use for non-technical users who can't run their own instances.
- Handling search and discovery across a distributed network efficiently.
- Implementing effective spam prevention and moderation in a distributed system.

## Key Focus Areas
1. Designing an efficient and robust federation protocol
2. Creating a user-friendly interface for both event organizers and attendees
3. Developing tools to make instance setup and maintenance as simple as possible
4. Building features that incentivize users to run and maintain their own instances

## Architecture Outline

### 1. Server Application
- **Language**: Rust
- **Responsibilities**: 
  - Hosting events and user data
  - Handling API requests
  - Managing instance synchronization

### 2. Cassandra Instances
- **Responsibilities**:
  - Storing and replicating data
  - Handling conflict resolution
- **Scaling**:
  - need a small cluster in the US
  - expand to several Zones globally
  - Institutional users will want a private cluster
  - **Challenges**:
  - How do we federate public and private so that users may participate in both public and institutaion events.

### 3. Federation Protocol
- **Responsibilities**:
  - Cross-instance lookups
  - limited data synchronization between public and private instances

### 4. Web Interface / Native App
- **Technologies**: Vue.js, tauri in rust, possibly Quaser.js
- **Responsibilities**:
  - User interaction with the platform
  - Event creation, management, and discovery
  - User registration and authentication

### 5. API Layer
- **Type**: REST API
- **Responsibilities**:
  - Exposing endpoints for frontend and third-party integrations
  - Handling CRUD operations for events and user data

### 6. Authentication and Authorization
- **Technologies**: OAuth2, JWT
- **Responsibilities**:
  - Secure user authentication using 
  - Role-based access control

### 7. Monitoring and Logging
- **Technologies**: Prometheus, Grafana
- **Responsibilities**:
  - System health monitoring
  - Logging and error tracking

### 8. Instance Management
- **Technologies**: Ansible, Docker, Kubernetes
- **Responsibilities**:
  - Tools for easy instance setup and maintenance
  - Configuration management
  - Instance discovery and registration


## Data Model
### Queries we'd like to make

1. Create/register user
1. update user
1. delete user
1. get user
1. login
1. logout
1. create event
1. update event
1. delete event
1. Get all events for a given user
2. Get all events for a given user and a given time frame
1. get all events near me in a time frame
1. get all events near me in a time frame, with similar interests to me
1. get event by id
1. CRUD a group
1. Set owners of a group
1. set interests for a group
1. select all groups near me 
1. select all groups with a certain set of interest




## Problems with existing solutions:
* groups are ghost towns. people don't show up to events.
* $30 per month to host a group keeps the young, who have no money, the old who have limited funds, and the poor who have no funds, away from the table.
* existing platforms don't do a good job of helping people find events.
* regular events in a group form an in-group that keeps new people out.
* "they took my abandoned event and ran it as their own, because I didn't take my data with me when I left."
* support is anything but
* RSVP no-shows
* Meetup currently runs on three different interfaces that interact with each other with sometimes chaotic results.
* Groups focused on selling something
* Social acceptance of meeting strangers
* physical safety concerns
* harassment
* Many people want more spontanious events, "I'm running to the park for an hour, anyone want come hang out right now?"
* "it feels like Meetup works best for medium sized lecture-style groups and is less useful for small, unstructured groups."
* "It’s mostly geeks and the unpopular kids all getting together." Acceptance and inclusivity is a problem.


## Existing solutions:
* Meetup
* Eventbrite
* Facebook Events
* Nextdoor
* https://lu.ma/
* https://partiful.com/
