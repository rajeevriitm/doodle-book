use crate::schema::users;
use diesel::prelude::*;
use rocket_sync_db_pools::diesel;

#[derive(FromForm, Debug, Insertable, Clone)]
#[table_name = "users"]
pub struct SignupForm {
    username: String,
    email: String,
    password: String,
}
impl SignupForm {
    pub fn save_to_db(&self, conn: &mut diesel::PgConnection) -> QueryResult<usize> {
        diesel::insert_into(users::table).values(self).execute(conn)
    }
}
