#[macro_export] extern crate diesel;
use diesel::prelude::*;
use super::schema::users;

// Note: pwd actually represents the hash of the password
// privs is an eight bit integer representing individual privileges
// 1: 0 - comment
// 2: 0 - post
// 3: 0 - edit (post/comment)
// 4: 0 - delete own
// 5: 0 - delete others
// 6: 0 - access ap
// 7: 0 - make admin
// 8: 0 - root
#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub pwd: String,
    pub privs: i32,
}


#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub pwd: &'a str,
    pub privs: i32,
}
