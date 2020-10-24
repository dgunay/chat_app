//! The backing data store for the server.

use rand::prelude::*;
use std::{collections::HashMap, sync::RwLock};

use crate::user::{Id, ProtoUser, User};

// TODO: use a real database
type Database = RwLock<HashMap<Id, User>>;

pub struct Store {
    db: Database,
}

impl Store {
    pub fn new() -> Self {
        Store {
            // TODO: create a db
            db: RwLock::from(HashMap::new()),
        }
    }

    // TODO: if we have a collision (e.g. same user ID twice), what do we do?
    // error, or allow it and let a higher layer handle overwrite attempts?
    // Defaulting to allow.
    pub fn persist(&self, user: &User) -> Option<User> {
        let mut locked_db = self.db.write().unwrap(); // FIXME: don't unwrap, handle poisoned locks
        locked_db.insert(user.id.clone(), user.clone())
    }

    // TODO: this function must not have a race condition
    pub fn new_user(&self, proto_user: ProtoUser) -> Result<User, Error> {
        // Generate an ID that isn't already in the db
        let id = self.generate_new_id();

        let user = User {
            name: proto_user.name,
            id,
        };

        match self.persist(&user) {
            Some(found_user) => {
                // The user already existed... that's bad
                // put them back and then return err
                // FIXME: we really don't want the possibility of there being a
                // different user in the db, even if only for a split second.
                self.persist(&found_user);
                return Err(Error::UserAlreadyExists(found_user));
            }
            _ => {} // all good
        }

        Ok(user)
    }

    pub fn get(&self, id: &Id) -> Option<User> {
        let lock = self.db.read().unwrap();
        lock.get(id).cloned()
    }

    fn generate_new_id(&self) -> Id {
        let mut rng = thread_rng();
        let id: String = std::iter::repeat(())
            .map(|()| rng.sample(rand::distributions::Alphanumeric))
            .take(20)
            .collect();

        Id { 0: id }
    }
}

#[derive(Debug)]
pub enum Error {
    UserAlreadyExists(User),
}
