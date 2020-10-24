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