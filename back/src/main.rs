#[macro_use] extern crate rocket;
#[macro_use] extern crate dotenv_codegen;

mod controllers;
mod services;
mod model;

use crate::controllers::login_controller::login;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/login", routes![login])
}
