use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2;
use dotenv::dotenv;

use super::DbConn;
use super::schema::*;
use super::models::*;
use std::env;
use argon2::{
    password_hash::{
	rand_core::OsRng,
	PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};

#[derive(Debug)]
pub enum DbConnError {
    UrlFailed,
    ConnectionFailed,
}

pub enum AuthError {
    Unauthorized,
    DbFailed,
    TokenFailed,
    Other,
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

pub fn create_new_user<'a>(conn: &PgConnection, uname: &'a str, pwd: &'a str) -> Result<User, AuthError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let pwd_hash = match argon2.hash_password(pwd.as_bytes(), &salt) {
	Ok(n) => n.to_string(),
	Err(_) => {
	    return Err(AuthError::Other);
	}
    };
    let new_user = NewUser {
	name: uname,
	pwd: pwd_hash.as_str(),
	privs: 0b1011_0000,
    };
    match diesel::insert_into(users::table).values(&new_user).get_result(conn) {
        Ok(n) => Ok(n),
        Err(_) => Err(AuthError::DbFailed),
    }
    
}

pub fn authenticate_user(
    conn: &DbConn,
    uname: String,
    pwd: String,
) -> Result<String, AuthError> {
    Ok(String::new())
}
