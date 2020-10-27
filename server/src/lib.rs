#![feature(proc_macro_hygiene, decl_macro)]

use rocket::error::LaunchError;

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_derive_newtype;

pub mod schema;
// pub mod models;

pub mod conversation;
mod message;
pub mod routes;
pub mod store;
pub mod user;

pub struct Server {
    pub rocket: rocket::Rocket,
}

impl Server {
    pub fn new(storage: store::Store) -> Self {
        Self {
            rocket: rocket::ignite()
                .mount("/", routes![routes::get_user, routes::new_user])
                .manage(storage),
        }
    }

    pub fn run(self) -> LaunchError {
        self.rocket.launch()
    }
}
