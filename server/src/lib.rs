#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
mod message;
pub mod routes;
mod store;
pub mod user;

// TODO: remove anyhow if we don't use it
// use anyhow::Result;

pub struct Server {
    pub rocket: rocket::Rocket,
}

impl Server {
    pub fn new() -> Self {
        Self {
            rocket: rocket::ignite()
                .mount(
                    "/",
                    routes![routes::hello, routes::get_user, routes::new_user],
                )
                .manage(store::Store::new()),
        }
    }

    pub fn run(self) {
        self.rocket.launch();
    }
}
