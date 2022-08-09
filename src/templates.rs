use std::io::Cursor;

use askama::Template;
use rocket::http::ContentType;
use rocket::http::Status;
use rocket::response::Responder;
use rocket::response::Response;

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

#[derive(Template)]
#[template(path = "about.html")]
pub struct AboutTemplate {
    about_text: String,
}

impl AboutTemplate {
    pub fn new(about_text: String) -> AboutTemplate {
	AboutTemplate { about_text }
    }
}

impl<'r, 'o: 'r> Responder<'r, 'o> for AboutTemplate {
    fn respond_to(self, _request: &'r rocket::Request<'_>) -> rocket::response::Result<'o> {
        match self.render() {
            Ok(n) => {
                let response = Response::build()
                    .header(ContentType::HTML)
                    .sized_body(n.len(), Cursor::new(n))
                    .finalize();
                Ok(response)
            }
            Err(_) => Err(Status::InternalServerError),
        }
    }
}
impl<'r, 'o: 'r> Responder<'r, 'o> for IndexTemplate {
    fn respond_to(self, _request: &'r rocket::Request<'_>) -> rocket::response::Result<'o> {
        match self.render() {
            Ok(n) => {
                let response = Response::build()
                    .header(ContentType::HTML)
                    .sized_body(n.len(), Cursor::new(n))
                    .finalize();
                Ok(response)
            }
            Err(_) => Err(Status::InternalServerError),
        }
    }
}
