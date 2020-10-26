//! A message in transit and at rest. Should be serializable to JSON.

use chrono::prelude::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{conversation::ConversationId, user::User};

#[derive(Serialize, Deserialize)]
pub struct Message {
    from: User,
    to: ConversationId,
    body: String,
    timestamp: DateTime<Utc>,
}
