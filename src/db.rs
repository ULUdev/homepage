use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2;
use dotenv::dotenv;

use super::DbConn;
use super::schema::*;
use super::models::*;
use super::tokenstore::TokenStore;
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

#[derive(Debug)]
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

/// verify the data the user supplied and return a token that is stored on the
/// server
pub fn authenticate_user(
    conn: &PgConnection,
    uname: String,
    pwd: String,
    store: &mut TokenStore,
) -> Result<String, AuthError> {
    let argon2 = Argon2::default();
    let parsed_hash = match PasswordHash::new(&pwd) {
	Ok(n) => n,
	Err(_) => {return Err(AuthError::Other);}
    };
    if argon2.verify_password(pwd.as_bytes(), &parsed_hash).is_ok() {
	// generate new token, store it in the database and return it.
	let matched_users = match users::table.filter(users::dsl::name.eq(uname)).limit(1).load::<User>(conn) {
	    Ok(n) => n,
	    Err(_) => {
		return Err(AuthError::DbFailed);
	    }
	};
	let uid: u64 = matched_users[0].id.try_into().unwrap();
	let token = store.new_token(uid);
	Ok(String::new())
    } else {
	Err(AuthError::Unauthorized)
    }
}
