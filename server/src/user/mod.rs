use anyhow::Error;
use anyhow::Result;
use derive_more::Display;
use rocket::request::FromParam;
use serde::{Deserialize, Serialize};

mod protouser;
pub use protouser::ProtoUser;

use diesel::{Insertable, Queryable};

// our db schema
use crate::schema::users;

/// A user.
#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Queryable, Insertable)]
pub struct User {
    pub name: UserName,
    pub id: UserId,
}

// TODO: macro to derive FromParam?

/// Copy-on-write string representing a user ID
// TODO: add validation semantics I guess
#[derive(DieselNewType, Serialize, Deserialize, Hash, Eq, PartialEq, Clone, Debug, Display)]
pub struct UserId(pub String);

/// Things that can go wrong when creating a user's `UserId`
#[derive(Debug)]
pub enum UserIdError {
    InvalidUserId,
}

impl<'a> FromParam<'a> for UserId {
    type Error = Error;

    fn from_param(param: &'a rocket::http::RawStr) -> Result<Self> {
        let string = param.percent_decode()?;
        Ok(Self(string.to_string()))
    }
}

/// A user's display name.
#[derive(DieselNewType, Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct UserName(pub String);

#[cfg(test)]
mod tests {
    #[test]
    fn user_id_validation() {
        // todo!("Validate user IDs")
    }
}
