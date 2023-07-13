// use diesel::insert_into;
// use rocket_sync_db_pools::diesel::*;
use diesel::prelude::*;
// use rocket::serde::{Deserialize, Serialize};
use crate::model::user::User;
use crate::schema::{drawings, relationships, users};
use crate::services::Paginator;
use rocket::form::{self, Error};
use rocket_sync_db_pools::diesel;
use serde::Serialize;
use std::time::SystemTime;
// type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;
#[derive(Debug, Associations, Identifiable, Queryable, Serialize)]
#[diesel(belongs_to(User))]
pub struct Drawing {
    id: i32,
    pub points: String,
    width: i32,
    created_at: SystemTime,
    pub user_id: i32,
}
impl Drawing {
    pub fn user_drawings(
        user: &User,
        page: i64,
        conn: &mut diesel::PgConnection,
    ) -> QueryResult<Vec<Drawing>> {
        Drawing::belonging_to(user)
            .paginate(page)
            .order(drawings::created_at.desc())
            .load(conn)
    }
    pub fn find_drawing(id: i32, conn: &mut diesel::PgConnection) -> QueryResult<Drawing> {
        drawings::table.find(id).first(conn)
    }
    pub fn delete_drawing(id: i32, conn: &mut diesel::PgConnection) -> QueryResult<usize> {
        diesel::delete(drawings::table)
            .filter(drawings::id.eq(id))
            .execute(conn)
    }
    pub fn home(
        user: &User,
        page: i64,
        conn: &mut diesel::PgConnection,
    ) -> QueryResult<Vec<(User, Drawing)>> {
        users::table
            .inner_join(drawings::table)
            .left_outer_join(relationships::table.on(relationships::following_id.eq(users::id)))
            .filter(drawings::user_id.eq(user.id))
            .or_filter(relationships::follower_id.eq(user.id))
            .paginate(page)
            .order(drawings::created_at.desc())
            .select((users::all_columns, drawings::all_columns))
            .load(conn)

        // users::table
        //     .inner_join(dra  wings::table)
        //     .filter(users::id.eq(user.id))
        //     .paginate(page)
        //     .order(drawings::created_at.desc())
        //     .load(conn)
    }
    #[cfg(test)]
    pub fn delete_all(conn: &mut diesel::PgConnection) {
        diesel::delete(drawings::table)
            .execute(conn)
            .expect("to delete");
    }
}
#[derive(FromForm)]
pub struct DrawingForm<'a> {
    #[field(validate=check_points_format())]
    pub points: &'a str,
    width: i32,
}
fn check_points_format<'v>(string: &str) -> form::Result<'v, ()> {
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
#[derive(Insertable, Debug)]
#[diesel(table_name = drawings)]
pub struct NewDrawing {
    points: String,
    user_id: i32,
    width: i32,
}
impl<'a> DrawingForm<'a> {
    pub fn get_new_drawing(&self, user_id: i32) -> NewDrawing {
        NewDrawing {
            points: self.points.to_owned(),
            width: self.width,
            user_id,
        }
    }
}
