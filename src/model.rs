// use diesel::insert_into;
// use rocket_sync_db_pools::diesel::*;
use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};
use rocket_sync_db_pools::{database, diesel};
#[database("doodles")]
pub struct Db(diesel::PgConnection);
use super::schema::drawings;
use std::time::SystemTime;
// type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;
#[derive(Debug, Queryable)]
pub struct Drawing {
    id: i32,
    pub points: String,
    created_at: SystemTime,
    updated_at: SystemTime,
}
#[derive(Insertable, FromForm)]
#[table_name = "drawings"]
pub struct NewDrawing {
    pub points: String,
}
impl NewDrawing {
    pub fn save_to_db(&self, conn: &mut diesel::PgConnection) -> QueryResult<Drawing> {
        diesel::insert_into(drawings::table)
            .values(self)
            .get_result(conn)
    }
}
