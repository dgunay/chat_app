//! A chain of `Message`s between 2 or more people

use crate::{message::Message, user::User};
use serde::{Deserialize, Serialize};

// TODO: how can we make sure this is ergonomically done from a DB perspective?
// e.g. what db relationship do we use to express a conversation?
// - store messages separately and match on them? NO, because then an identical
//   group of participants can't have multiple different chatrooms
// - A table of conversations with their participants, and tag the conversation
//   by kind? (e.g. DM, or a group chat with the title xyz)

/// A conversation consisting of users and the messages they've sent, in order.
pub struct Conversation {
    // TODO: can this structure support maybe Roles? maybe leave that as out of scope
    name: Option<String>,
    id: ConversationId,

    participants: Vec<User>,

    // TODO: some kind of lazy-loadable streaming type would be a good fit here.
    messages: Vec<Message>,
}

#[derive(DieselNewType, Serialize, Deserialize, Debug)]
pub struct ConversationId(String);
