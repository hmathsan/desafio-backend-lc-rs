#[macro_use] extern crate rocket;
#[macro_use] extern crate dotenv_codegen;

mod controllers;
mod services;
mod model;
mod repositories;

use crate::{
    controllers::{
        login_controller::login, 
        cards_controller::{get_all_cards, post_create_new_card, put_update_card, delete_card}
    },
    repositories::RepositoryFairing
};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(RepositoryFairing)
        .mount("/login", routes![login])
        .mount("/cards", routes![get_all_cards, post_create_new_card, put_update_card, delete_card])
}
