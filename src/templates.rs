use std::io::Cursor;
use askama::Template;
use rocket::response::Responder;
use rocket::Response;
use rocket::http::{Status, ContentType};

#[derive(Debug)]
pub struct LoginInfo {
    pub loggedin: bool,
    pub username: Option<String>,
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    description: String,
    login_info: LoginInfo,
}

impl IndexTemplate {
    pub fn new(description: String, login_info: LoginInfo) -> IndexTemplate {
        IndexTemplate {
            description,
            login_info,
        }
    }
}

#[derive(Template)]
#[template(path = "about.html")]
pub struct AboutTemplate {
    about_text: String,
    login_info: LoginInfo,
}

impl AboutTemplate {
    pub fn new(about_text: String, login_info: LoginInfo) -> AboutTemplate {
        AboutTemplate {
            about_text,
            login_info,
        }
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

