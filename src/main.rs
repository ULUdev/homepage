#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
use diesel::pg::PgConnection;
use rocket::form::Form;
use rocket::fs::{FileServer, NamedFile};
use rocket::http::{Cookie, CookieJar, Status};
use rocket::outcome::Outcome;
use rocket::request::FromRequest;
use rocket::request::Request;
use rocket::response::Redirect;
use rocket::time::Duration;
use rocket::State;
use rocket::request;
use std::ops::Deref;
use std::path::Path;
use std::sync::{Arc, Mutex};
use templates::IndexTemplate;

pub mod ap;
pub mod db;
pub mod models;
pub mod schema;
pub mod templates;
pub mod tokenstore;
use templates::*;
use tokenstore::TokenStore;

use ap::ApResponse;

pub struct DbConn(diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<PgConnection>>);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for DbConn {
    type Error = ();
    async fn from_request(request: &'r Request<'_>) -> request::Outcome<DbConn, Self::Error> {
        let pool = request
            .guard::<&'r State<diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<PgConnection>>>>()
            .await;
        match pool {
            Outcome::Success(conn) => match (*conn).get() {
                Ok(c) => Outcome::Success(DbConn(c)),
                Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
            },
            _ => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

impl Deref for DbConn {
    type Target = PgConnection;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(FromForm)]
struct Login<'r> {
    uname: &'r str,
    pwd: &'r str,
}

#[get("/")]
async fn index(dbcon: DbConn) -> IndexTemplate {
    let content = match db::get_content_entry(&dbcon, String::from("description")) {
        Ok(n) => match String::from_utf8(n.content_inner) {
            Ok(k) => k,
            Err(_) => String::from("default"),
        },
        Err(_) => String::from("default"),
    };
    templates::IndexTemplate::new(content)
}

#[get("/projects")]
async fn projects() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/pages/projects.html"))
        .await
        .ok()
}

#[get("/about")]
async fn about() -> AboutTemplate {
    // TODO: needs an actual database integration
    AboutTemplate::new(String::new())
}

// TODO: implement admin panel
// #[get("/ap")]
// async fn admin_panel() -> AdminTemplate {
//
// }

#[get("/login")]
async fn login_page() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/pages/login.html"))
        .await
        .ok()
}

#[get("/ap")]
fn admin_panel(
    cookies: &CookieJar<'_>,
    dbcon: DbConn,
    token_store: &State<Arc<Mutex<TokenStore>>>,
) -> ApResponse {
    match cookies.get_private("token") {
	Some(token) => {
	    let ts = token_store.inner().lock().unwrap();
	    let token: u64 = match token.value().parse() {
		Ok(n) => n,
		Err(_) => {
		    todo!();
		}
	    };
	    let uid = ts.get_id(&token);
	    let uname = match db::get_uname(&dbcon, token) {
		Some(n) => n,
		None => {
		    return ApResponse::new_error();
		}
	    };
	    ApResponse::new(uname)
	},
	None => ApResponse::new_error(),
    }
}

#[post("/auth", data = "<login>")]
fn authenticate(
    cookies: &CookieJar<'_>,
    login: Form<Login<'_>>,
    dbcon: DbConn,
    token_store: &State<Arc<Mutex<TokenStore>>>,
) -> Redirect {
    let mut ts = token_store.inner().lock().unwrap();
    // get some token
    let token = match db::authenticate_user(
        &dbcon,
        login.uname.to_string(),
        login.pwd.to_string(),
        &mut *ts,
    ) {
        Ok(n) => n,
        Err(_) => {
            //return Status::InternalServerError;
            todo!();
        }
    };
    // store the token as a cookie on the client
    let n_tok = token.clone();
    cookies.add_private(
        Cookie::build("token", n_tok.clone())
            .max_age(Duration::days(1))
            .finish(),
    );

    Redirect::to(uri!("/ap"))
}

#[catch(404)]
async fn not_found() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/pages/404.html"))
        .await
        .ok()
}

#[launch]
fn rocket() -> _ {
    let token_store = Arc::new(Mutex::new(TokenStore::new()));
    rocket::build()
        .mount(
            "/",
            routes![index, authenticate, login_page, projects, about, admin_panel],
        )
        .mount("/static", FileServer::from("static/"))
        .mount("/dist", FileServer::from("dist/"))
        .register("/", catchers![not_found])
        .manage(db::establish_connection())
        .manage(token_store)
}

#[cfg(test)]
mod test {
    use super::*;
    use rocket::http::Status;
    use rocket::local::blocking::Client;
    use rocket::Build;
    use rocket::Rocket;

    fn test_rocket() -> Rocket<Build> {
        let token_store = Arc::new(Mutex::new(TokenStore::new()));
        rocket::build()
            .mount("/", routes![index, authenticate, login_page, projects])
            .mount("/static", FileServer::from("static/"))
            .mount("/dist", FileServer::from("dist/"))
            .register("/", catchers![not_found])
            .manage(token_store)
    }

    // #[test]
    // fn test_index() {
    //     let client = Client::tracked(test_rocket()).expect("valid rocket instance");
    //     let response = client.get(uri!(super::index)).dispatch();
    //     assert_eq!(response.status(), Status::Ok);
    // }

    #[test]
    fn test_projects() {
        let client = Client::tracked(test_rocket()).expect("valid rocket instance");
        let response = client.get(uri!(super::projects)).dispatch();
        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn test_static() {
        let client = Client::tracked(test_rocket()).expect("valid rocket instance");
        let response = client.get(uri!("/static/pages/projects.html")).dispatch();
        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn test_js() {
        let client = Client::tracked(test_rocket()).expect("valid rocket instance");
        let response = client.get(uri!("/dist/navbar-bundle.js")).dispatch();
        assert_eq!(response.status(), Status::Ok);
    }
}
