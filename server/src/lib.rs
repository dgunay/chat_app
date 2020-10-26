#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
mod message;
pub mod routes;
mod store;
pub mod user;

pub mod conversation;

// TODO: remove anyhow if we don't use it
// use anyhow::Result;

pub struct Server {
    pub rocket: rocket::Rocket,
}

impl Server {
    pub fn new(db_url: &str) -> Self {
        Self {
            rocket: rocket::ignite()
                .mount("/", routes![routes::get_user, routes::new_user])
                .manage(store::Store::new(db_url)),
        }
    }

    pub fn run(self) {
        self.rocket.launch();
    }
}
