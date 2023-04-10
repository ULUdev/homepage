#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
use diesel::pg::PgConnection;
use rocket::form::Form;
use rocket::fs::{FileServer, NamedFile};
use rocket::http::{Cookie, CookieJar, Status};
use rocket::outcome::Outcome;
use rocket::request;
use rocket::request::FromRequest;
use rocket::request::Request;
use rocket::response::Redirect;
use rocket::time::Duration;
use rocket::State;
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
//pub mod askama_rocket;
//use askama_rocket::*;
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

// helper function
fn create_login_info(dbcon: &DbConn, ts: &TokenStore, cookies: &CookieJar<'_>) -> LoginInfo {
    let mut username: Option<String> = None;
    let mut loggedin: bool = false;
    if let Some(cookie) = cookies.get_private("token") {
        let token: usize = cookie.value().parse().unwrap();
	if let Some(uid) = ts.get_id(&token) {
            loggedin = true;
            username = db::get_uname(dbcon, *uid);
	}
    }
    LoginInfo { username, loggedin }
}

#[get("/")]
async fn index(
    dbcon: DbConn,
    token_store: &State<Arc<Mutex<TokenStore>>>,
    cookies: &CookieJar<'_>,
) -> IndexTemplate {
    let ts = token_store.inner().lock().unwrap();
    let content = match db::get_content_entry(&dbcon, String::from("description")) {
        Ok(n) => match String::from_utf8(n.content_inner) {
            Ok(k) => k,
            Err(_) => String::from("default"),
        },
        Err(_) => String::from("default"),
    };
    // eprintln!("{:?}", create_login_info(&dbcon, &ts, cookies));
    templates::IndexTemplate::new(content, create_login_info(&dbcon, &ts, cookies))
}

// TODO: make a template out of this
#[get("/projects")]
async fn projects() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/pages/projects.html"))
        .await
        .ok()
}

#[get("/about")]
async fn about(
    dbcon: DbConn,
    token_store: &State<Arc<Mutex<TokenStore>>>,
    cookies: &CookieJar<'_>,
) -> AboutTemplate {
    let ts = token_store.inner().lock().unwrap();
    // TODO: needs an actual database integration
    AboutTemplate::new(String::new(), create_login_info(&dbcon, &ts, cookies))
}

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
	    // we can just panic if parsing failed, as it will automatically
	    // return a 500
	    let token:  usize = token.value().parse().unwrap();
            let uid = ts.get_id(&token).unwrap();
            let uname = match db::get_uname(&dbcon, *uid) {
                Some(n) => n,
                None => {
                    return ApResponse::new_error(create_login_info(&dbcon, &ts, cookies));
                }
            };
            ApResponse::new(uname, create_login_info(&dbcon, &ts, cookies))
        }
        None => ApResponse::new_error(LoginInfo {
            loggedin: false,
            username: None,
        }),
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
        Err(e) => match e {
            db::AuthError::DbFailed => {
                return Redirect::to(uri!("/db_failed"));
            }
            db::AuthError::Unauthorized => {
                return Redirect::to(uri!("/auth_failed"));
            }
            db::AuthError::TokenFailed => {
                return Redirect::to(uri!("/login"));
            }
            db::AuthError::Other => {
                return Redirect::to(uri!("/auth_failed"));
            }
        },
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

#[get("/auth_failed")]
async fn auth_failed() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/pages/auth_failed.html"))
        .await
        .ok()
}
#[get("/logout")]
fn logout(token_store: &State<Arc<Mutex<TokenStore>>>, cookies: &CookieJar<'_>) -> Redirect {
    let mut ts = token_store.inner().lock().unwrap();
    if let Some(mut cookie) = cookies.get_private("token") {
        let token: usize = cookie.value().parse().unwrap();
        if ts.has_token(&token) {
            ts.delete_token(&token);
        }
        cookie.make_removal();
    }
    Redirect::to(uri!("/"))
}

#[launch]
fn rocket() -> _ {
    let token_store = Arc::new(Mutex::new(TokenStore::new()));
    rocket::build()
        .mount(
            "/",
            routes![
                index,
                authenticate,
                login_page,
                projects,
                about,
                admin_panel,
                auth_failed,
                logout
            ],
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
