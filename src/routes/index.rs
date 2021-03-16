use rocket_contrib::templates::Template;

#[derive(serde::Serialize)]
struct TemplateContext {
    name: String,
}

#[get("/")]
pub fn get() -> Template {
    let context = TemplateContext {
        name: "Hirad".to_string(),
    };
    Template::render("index", &context)
}
