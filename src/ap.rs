use std::io::Cursor;
use askama::Template;
use crate::LoginInfo;
use rocket::response::Responder;
use rocket::Response;
use rocket::http::ContentType;
use rocket::http::Status;

enum ApResponseKind {
    Success,
    AuthFailure,
}

#[derive(Template)]
#[template(path = "admin_panel.html")]
pub struct ApResponse {
    uname: String,
    kind: ApResponseKind,
    login_info: LoginInfo,
}

impl ApResponse {
    pub fn new(uname: String, login_info: LoginInfo) -> ApResponse {
        ApResponse {
            uname,
            kind: ApResponseKind::Success,
            login_info,
        }
    }

    pub fn new_error(login_info: LoginInfo) -> ApResponse {
        ApResponse {
            uname: String::new(),
            kind: ApResponseKind::AuthFailure,
            login_info,
        }
    }
}

impl<'r, 'o: 'r> Responder<'r, 'o> for ApResponse {
    fn respond_to(self, _request: &'r rocket::Request<'_>) -> rocket::response::Result<'o> {
        match self.kind {
            ApResponseKind::Success => match self.render() {
                Ok(n) => {
                    let response = Response::build()
                        .header(ContentType::HTML)
                        .sized_body(n.len(), Cursor::new(n))
                        .finalize();
                    Ok(response)
                }
                Err(_) => Err(Status::InternalServerError),
            },
            ApResponseKind::AuthFailure => Err(Status::Forbidden),
        }
    }
}
