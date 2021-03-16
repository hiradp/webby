#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

mod routes;

use rocket_contrib::json::JsonValue;
use rocket_contrib::templates::Template;

#[derive(serde::Serialize)]
struct TemplateContext {
    name: String,
}

#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

#[get("/")]
fn index() -> Template {
    let context = TemplateContext { name: "Hirad".to_string() };
    Template::render("index", &context)
}

pub fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/api", routes![routes::version::get_version])
        .mount("/", routes![index])
        .attach(Template::fairing())
        .register(catchers![not_found])
}
