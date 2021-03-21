use rocket_contrib::json::JsonValue;

use crate::SETTINGS;

#[get("/version")]
pub fn get() -> JsonValue {
    json!({
    "debug": SETTINGS.debug,
    "version": env!("CARGO_PKG_VERSION"),
    })
}
