use rand::seq::IteratorRandom;
use rocket_contrib::templates::Template;

use crate::SETTINGS;

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
    let mut rng = rand::thread_rng();
    let choices = &SETTINGS.meta.favicons;
    let favicon = choices.chars().choose(&mut rng).unwrap().to_string();
    println!("Randomly chosen favicon {}", favicon);

    let context = TemplateContext {
        first_name: (&SETTINGS.person.first_name).clone(),
        last_name: (&SETTINGS.person.last_name).clone(),
        job: Job {
            company: (&SETTINGS.job.company).clone(),
            link: (&SETTINGS.job.url).clone(),
            title: (&SETTINGS.job.title).clone(),
        },
        favicon,
    };
    Template::render("index", &context)
}
