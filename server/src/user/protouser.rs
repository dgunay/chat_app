use serde::Deserialize;

use super::UserName;

/// A user, but before being assigned an ID and being added to the db.
#[derive(Deserialize)]
pub struct ProtoUser {
    pub name: UserName,
}

// #[derive(Debug)]
// pub enum Error {
//     UserIdAlreadyExists(Id),
// }
