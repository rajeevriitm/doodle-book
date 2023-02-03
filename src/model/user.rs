use std::hash;

use crate::schema::users;
use argon2::PasswordHasher;
use diesel::prelude::*;
use rocket::form::validate;
use rocket_sync_db_pools::diesel;

#[derive(FromForm, Debug, Insertable, Clone)]
#[table_name = "users"]
pub struct SignupForm {
    #[field(validate=len(1..))]
    username: String,
    #[field(validate=contains('@'))]
    email: String,
    #[field(validate=len(2..))]
    pub password: String,
}
impl SignupForm {
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
