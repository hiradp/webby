#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

mod models;
mod routes;
mod settings;

use rocket_contrib::json::JsonValue;
use rocket_contrib::templates::Template;

use routes::api::v1;
use routes::index;

use crate::settings::SETTINGS;

#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

pub fn rocket() -> rocket::Rocket {
    // Print out our settings
    println!("{:?}", SETTINGS.debug);

    rocket::ignite()
        .mount("/api/v1", routes![v1::version::get])
        .mount("/", routes![index::get])
        .attach(Template::fairing())
        .register(catchers![not_found])
}
