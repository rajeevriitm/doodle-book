// use diesel::insert_into;
// use rocket_sync_db_pools::diesel::*;
use diesel::prelude::*;
// use rocket::serde::{Deserialize, Serialize};
use crate::schema::drawings;
use rocket::form::{self, Error};
use rocket_sync_db_pools::diesel;
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
    #[field(validate=check_points_format())]
    pub points: String,
}
fn check_points_format<'v>(string: &String) -> form::Result<'v, ()> {
    let res = serde_json::from_str::<Vec<Vec<[i32; 2]>>>(string);
    res.map_or(
        Err(Error::validation("Invalid points").into()),
        |collection| {
            let count = collection.iter().flatten().count();
            if count >= 1 {
                Ok(())
            } else {
                Err(Error::validation("Canvas cant be empty").into())
            }
        },
    )
    // println!("{:?}", &res);
}
impl NewDrawing {
    pub fn save_to_db(&self, conn: &mut diesel::PgConnection) -> QueryResult<usize> {
        diesel::insert_into(drawings::table)
            .values(self)
            .execute(conn)
    }
}
