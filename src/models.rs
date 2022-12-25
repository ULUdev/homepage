use super::schema::*;

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

/// struct that represents the privileges a user can have
#[derive(Clone, Copy, PartialEq)]
pub struct UserPrivs {
    pub comment: bool,
    pub post: bool,
    pub edit: bool,
    pub delete_own: bool,
    pub delete_others: bool,
    pub admin_panel: bool,
    pub make_admin: bool,
    pub root: bool,
}

impl UserPrivs {
    /// create a new UserPrivs object
    pub fn new() -> UserPrivs {
        UserPrivs {
            comment: false,
            post: false,
            edit: false,
            delete_own: false,
            delete_others: false,
            admin_panel: false,
            make_admin: false,
            root: false,
        }
    }

    /// allow a user to comment
    pub fn comment(mut self, val: bool) -> Self {
        self.comment = val;
        self
    }

    /// allow a user to post
    pub fn post(mut self, val: bool) -> Self {
        self.post = val;
        self
    }

    /// allow a user to edit his existing posts
    pub fn edit(mut self, val: bool) -> Self {
        self.edit = val;
        self
    }

    /// allow a user to delete his posts
    pub fn delete_own(mut self, val: bool) -> Self {
        self.delete_own = val;
        self
    }

    /// allows a user to delete posts made by others
    pub fn delete_others(mut self, val: bool) -> Self {
        self.delete_others = val;
        self
    }

    /// allow a user to access the admin panel
    pub fn admin_panel(mut self, val: bool) -> Self {
        self.admin_panel = val;
        self
    }

    /// allow a user to make other users admin
    pub fn make_admin(mut self, val: bool) -> Self {
        self.make_admin = val;
        self
    }

    /// the user is root
    pub fn root(mut self, val: bool) -> Self {
        self.root = val;
        self
    }

    /// convert the object to an integer
    pub fn build(&self) -> u8 {
        let mut int: u8 = 0;
        match self.comment {
            true => int |= 0b00000001,
            false => (),
        }
        match self.post {
            true => int |= 0b00000010,
            false => (),
        }
        match self.edit {
            true => int |= 0b00000100,
            false => (),
        }
        match self.delete_own {
            true => int |= 0b00001000,
            false => (),
        }
        match self.delete_others {
            true => int |= 0b00010000,
            false => (),
        }
        match self.admin_panel {
            true => int |= 0b00100000,
            false => (),
        }
        match self.make_admin {
            true => int |= 0b01000000,
            false => (),
        }
        match self.root {
            true => int |= 0b10000000,
            false => (),
        }
        int
    }
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub pwd: &'a str,
    pub privs: i32,
}

#[derive(Queryable, Clone)]
pub struct ContentEntry {
    pub name: String,
    pub content_inner: Vec<u8>,
    pub mime_type: String,
}

#[derive(Insertable)]
#[table_name = "content"]
pub struct NewContentEntry<'a> {
    pub name: &'a str,
    pub content_inner: &'a [u8],
    pub mime_type: &'a str,
}
