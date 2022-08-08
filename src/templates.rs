use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    description: String,
}

impl IndexTemplate {
    pub fn new(description: String) -> IndexTemplate {
        IndexTemplate { description }
    }
}
