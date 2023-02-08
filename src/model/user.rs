use crate::schema::users;
use argon2::{PasswordHasher, PasswordVerifier};
use diesel::prelude::*;
use rocket::form::validate;
use rocket_sync_db_pools::diesel;
use serde::Serialize;

#[derive(FromForm, Debug, Insertable, Clone)]
#[table_name = "users"]
pub struct UserForm {
    #[field(default="Tom",validate=len(1..))]
    username: String,
    #[field(validate=contains('@'))]
    pub email: String,
    #[field(validate=len(2..))]
    pub password: String,
}
#[derive(FromForm, Debug, Queryable, Identifiable, Serialize)]
pub struct User {
    pub id: i32,
    username: String,
    email: String,
    password: String,
}
impl User {
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
