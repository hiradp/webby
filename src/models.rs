use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Person {
    pub first_name: String,
    pub last_name: String,
}

#[derive(Debug, Deserialize)]
pub struct Job {
    pub company: String,
    pub title: String,
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct Meta {
    pub favicons: String,
}
