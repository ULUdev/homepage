use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2;
use dotenv::dotenv;

use super::models::*;
use super::schema::*;
use super::tokenstore::TokenStore;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use std::env;

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

pub fn create_new_user<'a>(
    conn: &PgConnection,
    uname: &'a str,
    pwd: &'a str,
) -> Result<User, AuthError> {
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
        privs: UserPrivs::new()
            .comment(true)
            .post(true)
            .edit(true)
            .delete_own(true)
            .build().try_into().unwrap(),
    };
    match diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
    {
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
    let pwd_hash = match users::table
        .filter(users::dsl::name.eq(uname.clone()))
        .limit(1)
        .load::<User>(conn)
    {
        Ok(n) => n[0].pwd.clone(),
        Err(_) => {
            return Err(AuthError::DbFailed);
        }
    };
    //DEBUG
    let parsed_hash = match argon2::PasswordHash::new(&pwd_hash) {
        Ok(n) => n,
        Err(_) => {
            return Err(AuthError::Other);
        }
    };
    if argon2.verify_password(pwd.as_bytes(), &parsed_hash).is_ok() {
        // generate new token, store it in the database and return it.
        let matched_users = match users::dsl::users
            .filter(users::dsl::name.eq(uname))
            .limit(1)
            .load::<User>(conn)
        {
            Ok(n) => n,
            Err(_) => {
                return Err(AuthError::DbFailed);
            }
        };
        let uid: usize = matched_users[0].id.try_into().unwrap();
        let _token = store.new_token(uid);
        Ok(uid.to_string())
    } else {
        Err(AuthError::Unauthorized)
    }
}

pub fn get_content_entry(conn: &PgConnection, title: String) -> Result<ContentEntry, DbConnError> {
    match content::dsl::content
        .filter(content::dsl::name.eq(title))
        .limit(1)
        .load::<ContentEntry>(conn)
    {
        Ok(n) => Ok(n[0].clone()),
        Err(_) => Err(DbConnError::ConnectionFailed),
    }
}

pub fn get_uname(conn: &PgConnection, uid: usize) -> Option<String> {
    match users::dsl::users
        .filter(users::dsl::id.eq::<i32>(uid.try_into().unwrap()))
        .limit(1)
        .load::<User>(conn)
    {
        Ok(n) => Some(n[0].name.clone()),
        Err(_) => None,
    }
}
