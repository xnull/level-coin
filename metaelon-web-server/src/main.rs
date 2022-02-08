#[macro_use]
extern crate rocket;
extern crate core;

use std::borrow::{Borrow, BorrowMut};
use rocket::{
    post,
    response::content,
    routes,
    serde::{Deserialize, json::Json, Serialize},
};

#[derive(Deserialize, Serialize)]
struct User {
    id: usize,
    name: String,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/users/<id>")]
fn user(id: usize) -> Json<User> {
    let user_from_id = User { id, name: String::from("Gennady") };
    Json(user_from_id)
}

/**
1. user registration via:
      - social networks
      - digital signatures / public key  - wallets
      - certificates?
2. Shamir?
3. distributed generating secret key
 **/

/*
#[rocket::main]
async fn main() {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![user])
        .launch()
        .await;
}
*/

fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    println!("The value of x is: {}", x);
}

//#[launch]
//fn rocket() -> _ {
//    rocket::build()
//        .mount("/", routes![index])
//        .mount("/", routes![user])
//}

