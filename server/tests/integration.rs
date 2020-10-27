#[macro_use]
extern crate diesel;

use diesel::prelude::*;
use rocket::{http::ContentType, http::Status, local::Client, local::LocalResponse};
use server::user::User;
use server::{store::Store, Server};

// Run a local docker container on this machine with the test db in a blank
// state
const DB_URL: &str = "postgres://postgres:test@localhost:5432/postgres";

// FIXME: make the tests actually work

fn fixture_setup() -> Server {
    // Attain connection to db
    let connection = PgConnection::establish(DB_URL).unwrap();

    // Run the appropriate migrations to create our tables
    let migration = diesel_migrations::migration_from("../migrations/".into()).unwrap();
    migration.run(&connection).unwrap();

    // Assemble our Server
    let store = Store::new(connection);
    Server::new(store)
}

#[test]
fn user_not_found() {
    let fixture_server = fixture_setup();
    let rocket = fixture_server.rocket;
    let client = Client::new(rocket).expect("Valid rocket instance");

    // Get a bogus user and see if it 404's
    let response = client.get("/users/id/12345").dispatch();
    assert_eq!(response.status(), Status::NotFound);

    // TODO: teardown the db
}

#[test]
fn add_user() {
    let fixture_server = fixture_setup();
    let rocket = fixture_server.rocket;
    let client = Client::new(rocket).expect("Valid rocket instance");

    // Create a new user on the server
    let mut req = client.post("/users/new");
    req.add_header(ContentType::JSON);
    let response = req.body(r#"{ "name" : "Big Chungus" }"#).dispatch();
    assert_eq!(response.status(), Status::Ok);

    // Obtain our newly-created user
    let user = from_response(response);

    // Attempt to retrieve the newly-created user
    let response = client.get(format!("/users/id/{}", user.id)).dispatch();
    assert_eq!(response.status(), Status::Ok);
    let retrieved_user = from_response(response);
    assert_eq!(user, retrieved_user);
}

fn from_response(mut resp: LocalResponse) -> User {
    serde_json::from_str(resp.body().unwrap().into_string().unwrap().as_str()).unwrap()
}
