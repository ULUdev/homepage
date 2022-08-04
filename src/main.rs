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
use rocket::State;
use std::path::Path;

pub mod db;
pub mod schema;
pub mod models;

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
fn authenticate(cookies: &CookieJar<'_>, login: Form<Login<'_>>, dbcon: DbConn) {
    // get some token
    let token = 0;
    // store the token as a cookie on the client
    cookies.add_private(Cookie::new("auth_key", "<some token>"));

    // store the token in the database
    // db::authenticate_user(conn, login.uname.to_string(), login.pwd.to_string());
}

#[catch(404)]
async fn not_found() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/pages/404.html"))
        .await
        .ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, authenticate, login_page, projects])
        .mount("/static", FileServer::from("static/"))
        .register("/", catchers![not_found])
        .manage(db::establish_connection())
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
