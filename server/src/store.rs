//! The backing data store for the server.

use crate::{
    message::MessageId,
    schema::{messages, users},
};
use anyhow::{Context, Result};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use rand::prelude::*;
use std::{
    fmt::Debug,
    sync::{Mutex, RwLock},
};

use crate::{
    message::Message,
    user::{ProtoUser, User, UserId},
};

pub struct Store {
    db: Mutex<PgConnection>,
}

trait DataAccessObject<Entity, UniqueId> {
    fn retrieve(&self, uid: &UniqueId) -> Result<Entity>;
    fn persist(&self, entity: &Entity) -> Result<Entity>;
}

/// Interface to separate database access code from the higher-level API
// trait DataAccessObject {
//     fn retrieve_user(&self, uid: &UserId) -> Result<User>;
//     fn persist_user(&self, user: &User) -> Result<User>;
//     fn persist_message(&self, message: &Message) -> Result<Message>;
// }
use users::dsl::*;

impl DataAccessObject<User, UserId> for Store {
    fn retrieve(&self, uid: &UserId) -> Result<User> {
        let locked_db = self.db.lock().unwrap();
        users
            .filter(users::columns::id.eq(uid))
            .first(&*locked_db)
            .context(format!("Failed to find user (id: {:?})", uid))
    }

    fn persist(&self, entity: &User) -> Result<User> {
        let locked_db = self.db.lock().unwrap(); // FIXME: don't unwrap, handle poisoned locks
        diesel::insert_into(users::table)
            .values(entity)
            .get_result::<User>(&*locked_db)
            .context(format!("Failed to add new user {:?}", entity))
    }
}

use messages::dsl::*;

impl DataAccessObject<Message, MessageId> for Store {
    fn retrieve(&self, uid: &MessageId) -> Result<Message> {
        let locked_db = self.db.lock().unwrap();
        messages
            .filter(messages::columns::id.eq(uid))
            .first(&*locked_db)
            .context(format!("Failed to find user (id: {:?})", uid))
    }

    fn persist(&self, entity: &Message) -> Result<Message> {
        let locked_db = self.db.lock().unwrap(); // FIXME: don't unwrap, handle poisoned locks
        diesel::insert_into(messages::table)
            .values(entity)
            .get_result::<Message>(&*locked_db)
            .context(format!("Failed to add new message {:?}", entity))
    }
}

//     fn persist_message(&self, message: &Message) -> Result<Message> {
//         let mut locked_db = self.db.lock().unwrap(); // FIXME: don't unwrap, handle poisoned locks
//         todo!("Insert message")
//     }
// }

impl Store {
    pub fn new(connection: PgConnection) -> Self {
        Store {
            db: Mutex::new(connection),
        }
    }

    /// Proxy for calling `test_transaction` on the underlying connection object.
    pub fn test<T, E>(&self, f: fn() -> Result<T, E>) -> T
    where
        E: Debug,
    {
        self.db
            .lock()
            .expect("Failed to lock db")
            .test_transaction(f)
    }

    // TODO: if we have a collision (e.g. same user ID twice), what do we do?
    // error, or allow it and let a higher layer handle overwrite attempts?

    // TODO: this function must not have a race condition even if we distribute
    // the db - how can we guarantee that?
    pub fn new_user(&self, proto_user: ProtoUser) -> Result<User> {
        // Generate an ID that isn't already in the db
        let uid = self.generate_new_id();

        let user = User {
            name: proto_user.name,
            id: uid,
        };

        self.persist(&user)
    }

    pub fn get_user(&self, uid: &UserId) -> Option<User> {
        self.retrieve(uid).ok()
    }

    fn generate_new_id(&self) -> UserId {
        let mut rng = thread_rng();
        let uid: String = std::iter::repeat(())
            .map(|()| rng.sample(rand::distributions::Alphanumeric))
            .take(20)
            .collect();

        UserId { 0: uid }
    }
}

#[derive(Debug)]
pub enum Error {
    UserAlreadyExists(User),
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use crate::user::User;

    use super::{DataAccessObject, Store};

    struct FakeStore {
        // no db
    }

    impl DataAccessObject for FakeStore {
        fn get_user(&self, user: &User) -> Result<User> {
            todo!()
        }

        fn persist_user(&self, user: &User) -> Result<User> {
            todo!()
        }
    }

    #[test]
    fn add_user() {}
}
