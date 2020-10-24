//! A message in transit and at rest. Should be serializable to JSON.

use serde::{Deserialize, Serialize};

use crate::user::User;

#[derive(Serialize, Deserialize)]
pub struct Message {
    from: User,
    to: User,
    body: String,
}
