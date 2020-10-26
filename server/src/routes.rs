//! API routes. Each function corresponds to a route in the Rocket API.

use rocket::State;
use rocket_contrib::json::Json;

use crate::{message::Message, store::Error, user::ProtoUser};

use super::store::Store;
use super::user::{User, UserId};

#[get("/users/id/<id>")]
pub fn get_user(store: State<'_, Store>, id: UserId) -> Option<Json<User>> {
    println!("Received request for user");

    let user = store.inner().get_user(&id)?;
    Some(Json(user))
}

#[post("/users/new", format = "json", data = "<proto_user>")]
pub fn new_user(store: State<Store>, proto_user: Json<ProtoUser>) -> Result<Json<User>, Error> {
    Ok(Json(store.inner().new_user(proto_user.0)?))
}

//
#[post("/messages/store", format = "json", data = "<message>")]
pub fn store_message(store: State<Store>, message: Json<Message>) -> Result<Json<User>, Error> {
    todo!("Store message route")
    // store.inner().persist_message(message)
}
