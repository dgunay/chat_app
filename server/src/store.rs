//! The backing data store for the server.

use anyhow::Result;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use rand::prelude::*;
use std::{
    collections::HashMap,
    sync::{Mutex, RwLock},
};

use crate::{
    message::Message,
    user::{ProtoUser, User, UserId},
};

// TODO: use a real database
type Database = RwLock<HashMap<UserId, User>>;

pub struct Store {
    db: Mutex<PgConnection>,
}

impl Store {
    pub fn new(db_url: &str) -> Result<Self> {
        Ok(Store {
            db: Mutex::new(PgConnection::establish(db_url)?),
        })
    }

    // TODO: if we have a collision (e.g. same user ID twice), what do we do?
    // error, or allow it and let a higher layer handle overwrite attempts?
    // Defaulting to allow.
    pub fn persist_user(&self, user: &User) -> Option<User> {
        let mut locked_db = self.db.lock().unwrap(); // FIXME: don't unwrap, handle poisoned locks
        todo!("Insert user after creating the schema")
        // locked_db.
        // locked_db..insert(user.id.clone(), user.clone())
    }

    pub fn persist_message(&self, message: &Message) {
        let mut locked_db = self.db.lock().unwrap(); // FIXME: don't unwrap, handle poisoned locks
        todo!("Insert message")
    }

    // TODO: this function must not have a race condition
    pub fn new_user(&self, proto_user: ProtoUser) -> Result<User, Error> {
        // Generate an ID that isn't already in the db
        let id = self.generate_new_id();

        let user = User {
            name: proto_user.name,
            id,
        };

        match self.persist_user(&user) {
            Some(found_user) => {
                // The user already existed... that's bad
                // put them back and then return err
                // FIXME: we really don't want the possibility of there being a
                // different user in the db, even if only for a split second.
                self.persist_user(&found_user);
                return Err(Error::UserAlreadyExists(found_user));
            }
            _ => {} // all good
        }

        Ok(user)
    }

    pub fn get_user(&self, id: &UserId) -> Option<User> {
        let lock = self.db.lock().unwrap();
        todo!("get user from db")
        // lock.get(id).cloned()
    }

    fn generate_new_id(&self) -> UserId {
        let mut rng = thread_rng();
        let id: String = std::iter::repeat(())
            .map(|()| rng.sample(rand::distributions::Alphanumeric))
            .take(20)
            .collect();

        UserId { 0: id }
    }
}

#[derive(Debug)]
pub enum Error {
    UserAlreadyExists(User),
}
