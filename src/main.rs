#[macro_use]
extern crate rocket;
use rocket::fs::{FileServer, NamedFile};
use std::path::Path;

#[get("/")]
async fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/pages/index.html"))
        .await
        .ok()
}

#[get("/")]
async fn projects() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/pages/projects.html"))
        .await
        .ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/static", FileServer::from("static/"))
        .mount("/projects", routes![projects])
}
