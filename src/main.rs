#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[get("/version")]
fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

fn main() {
    rocket::ignite()
        .mount("/", routes![version])
        .launch();
}
