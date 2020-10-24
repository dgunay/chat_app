//! API routes. Each function corresponds to a route in the Rocket API.

use rocket::State;
use rocket_contrib::json::Json;

use crate::{store::Error, user::ProtoUser};

use super::store::Store;
use super::user::{Id, User};

#[get("/")]
pub fn hello() -> String {
    "Hello!".to_string()
}

#[get("/users/id/<id>")]
pub fn get_user(store: State<'_, Store>, id: Id) -> Option<Json<User>> {
    println!("Received request for user");

    let user = store.inner().get(&id)?;
    Some(Json(user))
}

#[post("/users/new", format = "json", data = "<proto_user>")]
pub fn new_user(store: State<Store>, proto_user: Json<ProtoUser>) -> Result<Json<User>, Error> {
    Ok(Json(store.inner().new_user(proto_user.0)?))
}
