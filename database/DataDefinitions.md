# OpenMeet Data Definitions

## Objects
There are some objects that are common through the OpenMeet app.


### Interests

Users have interests. Groups address interests.

### Users

Users have a username, password, email.  They also have a list of interests attached to their profile.

### User Profile

Descriptions of the user.  This is their presentation to the community how they wish to be seen.  
Profiles should help match users up with groups.

### Groups

Groups are collections of users.  Groups are defined by a set of interests. Groups host events.

### Events

Events are created and hosted by groups. They have start and end times, and a location.  They may be open to the public or private.
If private, the members of the group are the only users who can see the event.

### Event Attendees

Users can attend events.  They may only attend events that are open to the public or that they are a member of.
 Event Hostsare a special type of attendee.  They are members of the group that is hosting the event.  They are responsible for the event.

### Event RSVPs

RSVPs are a record of a user's intent to attend an event.  They are used to determine the number of people to expect at an event.


## Types of query


* get all interests
* create interest
* delete interest
* get interest by name
* create user
* create group
* user joins group
* user leaves group
* create event
* user attends event
* user unattends event
* user RSVPs for event
* user un-RSVPs for event
* get user profile
* get group profile
* get event profile
* get all events
* get all groups
* get all users
* get all events a user is attending
* get all groups a user is in
* get all events a group is hosting
* get all users in a group
* get all events in a group
* get all users attending an event


## for prompting:

Using this document, let's look at updating the schema @schema.cql. Use the @Web to become an expert at cql, and apache cassandra db schema related concerns.
Suggest a new schema that supports all the types of queries we'll be getting, and any additional data structures we'll need.
Make sure indexes make sense.  If we neeed to update an existing schema item, call it out speciffically so I can make an upgrade script.