#[macro_use] extern crate rocket;
#[macro_use] extern crate dotenv_codegen;

mod controllers;
mod services;
mod model;
mod repositories;

use crate::{
    controllers::{login_controller::login, cards_controller::get_all_cards},
    repositories::RepositoryFairing
};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(RepositoryFairing)
        .mount("/login", routes![login])
        .mount("/cards", routes![get_all_cards])
}
