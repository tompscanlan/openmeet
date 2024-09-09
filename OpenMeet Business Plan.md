# OpenMeet Business Plan

## 1. Executive Summary

OpenMeet is an open-source distributed group event platform designed to provide communities with alternatives to existing solutions. We aim to create a public utility for community building tools that will revolutionize how people organize and discover local events and interest groups. Our platform offers a unique approach by allowing users to run their own instances, which can sync with others to form a larger network, promoting data ownership, privacy, and community control. The business will also operate a public instance at openmeet.net, which users can connect to if they don't have a local instance and choose not to run one.

Key features:
- Self-hostable, open-source platform for getting communities together
- Distributed network for event organization and discovery
- Public instance available for users who prefer not to self-host
- Emphasis on data ownership and privacy

OpenMeet addresses critical issues in the current event platform market, such as high costs and data control concerns. By offering a free public service and the option for self-hosting, we aim to enable modern communites.

Our target market includes privacy-conscious individuals, community organizers, institutions looking to facilitate community building, and users of existing platforms that aren't meeting their needs. OpenMeet is positioned to provide something that could challenge centralized social platforms, which are catering to profits over people.

The business model combines open-source development with potential revenue streams from premium features, support on institutional instances, and hosting solutions for smaller institutions. This approach allows us to maintain the platform's core values while ensuring sustainable growth and development.


## 2. Company Description

OpenMeet is a technology company dedicated to revolutionizing the way communities organize and discover events. Founded in 2024, our mission is to provide an open, accessible, and privacy-focused platform for bringing people together.

Mission: To empower modern communities by providing free, open-source tools for event organization and discovery, while prioritizing data ownership and user privacy.

Vision: We envision a world where every community, regardless of size or resources, has access to powerful, customizable tools for connecting and organizing events without compromising on data privacy or control.

Core Values:
1. Openness: We believe in the power of open-source software to drive innovation and community collaboration.
2. Privacy: We prioritize user data protection and give individuals control over their information.
3. People are not products: We do not sell customer data, or serve ads.
4. Community-driven: We actively involve our user community in the development and improvement of our platform.
5. Inclusivity: We believe that everyone should have the opportunity to participate in community building, regardless of their resources.

OpenMeet's unique selling proposition lies in its distributed architecture, which allows for a decentralized network of event platforms. This approach sets us apart from traditional centralized services by offering:

- Greater data control and privacy for users and organizations
- Zero dependency on a single service provider
- Community-driven development and feature implementation

Our team consists of Tom Scanlan, an experienced software developer and operator, a small team of developers working on an MVP.  Additionally, we're leverging feedback from the community to build the platform. We are passionate about creating technology that serves the public good. When the MVP shows promise, we'll begin working with a legal team to ensure compliance and protect the platform and its users. By combining technical expertise with a deep understanding of community needs, OpenMeet is positioned to become a leading force in the evolution of event organization and discovery platforms.


## 3. Market Analysis
- Target Market
- Industry Trends
- Competitor Analysis

## 4. Product and Services
- Platform Features
- Unique Selling Proposition

## 5. Marketing and Sales Strategy
- Marketing Channels
- User Acquisition Strategy
- Retention Strategy

## 6. Business Model
- Revenue Streams
- Pricing Strategy

## 7. Operations Plan
- Technology Infrastructure
- Team Structure
- Key Partnerships

## 8. Financial Projections
- Startup Costs
- Revenue Forecasts
- Break-even Analysis

## 9. Funding Requirements

## 10. Milestones and Roadmap

## 11. Risk Analysis and Mitigation

## 12. Exit Strategy

---

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
