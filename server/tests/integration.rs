// #[macro_use]
// extern crate rocket;

use server::user::User;
use server::Server;

use rocket::{http::ContentType, http::Status, local::Client, local::LocalResponse};

#[test]
fn user_not_found() {
    let rocket = Server::new().rocket;
    let client = Client::new(rocket).expect("Valid rocket instance");

    // Get a bogus user and see if it 404's
    let response = client.get("/users/id/12345").dispatch();
    assert_eq!(response.status(), Status::NotFound);
}

#[test]
fn add_user() {
    let rocket = Server::new().rocket;
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
