use crate::schema::users;
use ::diesel::prelude::*;
use argon2::{PasswordHasher, PasswordVerifier};
use rocket_sync_db_pools::diesel;
use serde::Serialize;

#[derive(FromForm, Debug, Insertable, Clone)]
#[diesel(table_name = users)]
pub struct UserForm {
    #[field(default="Tom",validate=len(1..))]
    username: String,
    #[field(validate=contains('@'))]
    pub email: String,
    #[field(validate=len(2..))]
    pub password: String,
}
#[derive(FromForm, Debug, Queryable, Identifiable, Serialize, Clone)]
pub struct User {
    pub id: i32,
    username: String,
    email: String,
    password: String,
    profile_pic: Option<String>,
    profile_pic_width: Option<i32>,
    followers_count: i32,
    following_count: i32,
}
#[derive(AsChangeset, FromForm, Debug)]
#[diesel(table_name = users)]
pub struct UserUpdateForm {
    username: String,
    #[field(name = "points")]
    pub profile_pic: Option<String>,
    #[field(name = "width")]
    pub profile_pic_width: Option<i32>,
}
impl User {
    // pub fn count(conn: &mut diesel::PgConnection) -> i64 {
    //     users::table.count().get_result(conn).unwrap()
    // }
    pub fn find_user_with_name(
        username: &str,
        conn: &mut diesel::PgConnection,
    ) -> QueryResult<User> {
        users::table
            .filter(users::username.eq(username))
            .first(conn)
    }
    pub fn find(id: i32, conn: &mut diesel::PgConnection) -> QueryResult<User> {
        users::table.find(id).first(conn)
    }
    pub fn find_user_with_email(
        email: String,
        conn: &mut diesel::PgConnection,
    ) -> Result<User, diesel::result::Error> {
        users::table.filter(users::email.eq(email)).first(conn)
    }
    pub fn verify_password(&self, password: &str) -> Result<(), argon2::password_hash::Error> {
        let hash = argon2::password_hash::PasswordHash::new(&self.password)?;
        argon2::Argon2::default().verify_password(password.as_bytes(), &hash)
    }
    #[cfg(test)]
    pub fn delete_all(conn: &mut diesel::PgConnection) {
        diesel::delete(users::table).execute(conn).expect("delete");
    }
}
impl UserUpdateForm {
    pub fn update_user(&self, id: i32, conn: &mut diesel::PgConnection) -> QueryResult<usize> {
        diesel::update(users::table.find(id))
            .set(self)
            .execute(conn)
    }
}
impl UserForm {
    pub fn save_to_db(&self, conn: &mut diesel::PgConnection) -> QueryResult<usize> {
        diesel::insert_into(users::table).values(self).execute(conn)
    }
    pub fn hash_password(&mut self, salt: &str) -> Result<String, &'static str> {
        let argon = argon2::Argon2::default();
        argon
            .hash_password(&self.password.as_bytes(), salt)
            .map(|hash| hash.to_string())
            .or(Err("password error occured"))
    }
}
