use rocket_contrib::templates::Template;

#[derive(serde::Serialize)]
struct TemplateContext {
    first_name: String,
    last_name: String,
    job: Job,
    favicon: String,
}

#[derive(serde::Serialize)]
struct Job {
    company: String,
    link: String,
    title: String,
}

#[get("/")]
pub fn get() -> Template {
    let context = TemplateContext {
        first_name: "Hirad".to_string(),
        last_name: "Pourtahmasbi".to_string(),
        job: Job {
            company: "Jitterbit, Inc".to_string(),
            link: "https://jitterbit.com".to_string(),
            title: "Software Engineer".to_string(),
        },
        favicon: "🔥".to_string(),
    };
    Template::render("index", &context)
}
