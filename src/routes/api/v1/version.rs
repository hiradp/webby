use rocket_contrib::json::JsonValue;

#[get("/version")]
pub fn get() -> JsonValue {
    json!({ "version": env!("CARGO_PKG_VERSION") })
}
