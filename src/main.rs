#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use rocket::form::Form;
use rocket::fs::{FileServer, NamedFile};
use rocket::http::{Cookie, CookieJar, Status};
use rocket::outcome::Outcome;
use rocket::request;
use rocket::request::FromRequest;
use rocket::request::Request;
use rocket::time::Duration;
use rocket::State;
use std::ops::Deref;
use std::path::Path;
use std::sync::{Arc, Mutex};

pub mod db;
pub mod models;
pub mod schema;
pub mod tokenstore;
use tokenstore::TokenStore;

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
async fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/pages/index.html"))
        .await
        .ok()
}

#[get("/projects")]
async fn projects() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/pages/projects.html"))
        .await
        .ok()
}

#[get("/login")]
async fn login_page() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/pages/login.html"))
        .await
        .ok()
}

#[post("/auth", data = "<login>")]
fn authenticate(
    cookies: &CookieJar<'_>,
    login: Form<Login<'_>>,
    dbcon: DbConn,
    token_store: &State<Arc<Mutex<TokenStore>>>,
) -> String {
    let mut ts = token_store.inner().lock().unwrap();
    // get some token
    let token = match db::authenticate_user(
        &dbcon,
        login.uname.to_string(),
        login.pwd.to_string(),
	&mut *ts
    ) {
        Ok(n) => n,
        Err(e) => {
            return match e {
                db::AuthError::Unauthorized => String::from("<h1>Login Failed</h1>"),
                _ => String::from("<h1>Internal Server Error</h1>"),
            };
        }
    };
    // store the token as a cookie on the client
    let n_tok = token.clone();
    cookies.add_private(
        Cookie::build("token", n_tok.clone())
            .max_age(Duration::days(1))
            .finish(),
    );

    token
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
        .mount("/", routes![index, authenticate, login_page, projects])
        .mount("/static", FileServer::from("static/"))
        .register("/", catchers![not_found])
        .manage(db::establish_connection())
        .manage(token_store)
}

#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::http::Status;
    use rocket::local::blocking::Client;

    #[test]
    fn test_index() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get(uri!(super::index)).dispatch();
        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn test_projects() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get(uri!(super::projects)).dispatch();
        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn test_static() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get(uri!("/static/js/projects.js")).dispatch();
        assert_eq!(response.status(), Status::Ok);
    }
}
