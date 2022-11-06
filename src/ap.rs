use std::io::Cursor;

use askama::Template;
use rocket::{response::Responder , http::{ContentType, Status}};
use rocket::Response;


enum ApResponseKind {
    Success,
    AuthFailure,
}


#[derive(Template)]
#[template(path = "admin_panel.html")]
pub struct ApResponse {
    uname: String,
    kind: ApResponseKind,
}

impl ApResponse {
    pub fn new(uname: String) -> ApResponse {
	ApResponse { uname, kind: ApResponseKind::Success }
    }

    pub fn new_error() -> ApResponse {
	ApResponse { uname: String::new(), kind: ApResponseKind::AuthFailure }
    }
}

impl<'r, 'o: 'r> Responder<'r, 'o> for ApResponse {
    fn respond_to(self, request: &'r rocket::Request<'_>) -> rocket::response::Result<'o> {
	match self.kind {
	    ApResponseKind::Success => {
		match self.render() {
		    Ok(n) => {
			let response = Response::build()
			    .header(ContentType::HTML)
			    .sized_body(n.len(), Cursor::new(n))
			    .finalize();
			Ok(response)
		    },
		    Err(_) => Err(Status::InternalServerError),
		}
	    },
	    ApResponseKind::AuthFailure => Err(Status::Forbidden),
	}
    }
}
