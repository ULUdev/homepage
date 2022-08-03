use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2;
use dotenv::dotenv;
use super::DbConn;
use std::env;

#[derive(Debug)]
pub enum DbConnError {
    UrlFailed,
    ConnectionFailed,
}

pub enum AuthError {
    Unauthorized,
    DbFailed,
    TokenFailed,
}

pub fn establish_connection() -> r2d2::Pool<r2d2::ConnectionManager<PgConnection>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("no url provided");
    let manager = r2d2::ConnectionManager::<PgConnection>::new(database_url);
    match r2d2::Pool::builder().build(manager) {
	Ok(n) => n,
	Err(_) => panic!("failed to create pool"),
    }
    
}

pub fn authenticate_user(
    conn: &DbConn,
    uname: String,
    pwd: String,
) -> Result<String, AuthError> {
    Ok(String::new())
}
