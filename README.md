# Chat App

An attempt at designing and implementing a somewhat scalable chat application.

## Basic Requirements

- Realtime messaging
- Persistent accounts
- Friends lists
- Persistent chat history (to a limited degree perhaps)

### Stretch Goals

- Multi-user chatrooms (probably not hard to do tbh)
- Authentication (becomes required if this gets any traction at all)
- End-to-end encryption
- See who is logged in
- Read receipts
- (User) is typing...

## Architecture

Trying to get a better idea of how to architect distributed systems. This
probably won't be perfect but I think step one is to make something that is
ideally not too tightly coupled to upgrade in pieces.

I am trying to do the first pass at this without consulting any design studies 
or examples and then later go back and refine it.

### Client

The client can ideally be whatever implements my API, but we'll start with a 
simple web app. Login -> choose someone to talk to -> start talking.

### Server

The server should receive message payloads and fundamentally do 2 things:

1. Quickly send it to its destination(s)
2. Store the message for later reading

The server should make the history available in chunks to the client upon
request (so that you can log in and read old messages).

### Message Queue

To decouple sending vs receiving a message, a message queue should be used. This
will help avoid problems during high load (messages getting dropped).

### Communication

I think for simplicity's sake I'll initially go with JSON messages delivered
via HTTP but long-term it is probably worth considering more lightweight
alternatives.

### Database

Turns out choosing a database is really important, and doing this is making me think
a lot about what this app really would need as far as guarantees and 
performance/scalability characteristics from the db. I initially knee-jerk
reached for Postgres simply because I had heard a lot about it and it seemed
pretty buzzwordy, but I'm now aware that certain NoSQL dbs like MongoDB offer
better horizontal scalability in exchange for dropping certain guarantees
(such as atomicity).

I'll need to think about that. Off the top of my head, I think I could get away
with some message loss; I don't care if a message is dropped here and there so 
long as the user is made known of its disappearance. If the probability is in 
freak accident territory, maybe even we don't need to acknowledge it at all.

What I can't afford is losing users or conversations. I guess most technologies
can offer at least those guarantees for certain operations.

I may be overthinking this but I think maybe Cassandra would be good for this.
However at this point for the sake of compatibility with Diesel I'll be using
Postgres