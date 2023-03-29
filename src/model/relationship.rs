use crate::schema::{relationships, users};
use ::diesel::prelude::*;
use rocket_sync_db_pools::diesel::{self, PgConnection};
#[derive(FromForm)]
pub struct RelationshipForm {
    pub following_id: i32,
}
pub fn follow_user(
    follower_id: i32,
    following_id: i32,
    conn: &mut PgConnection,
) -> QueryResult<()> {
    conn.transaction(|conn| {
        diesel::insert_into(relationships::table)
            .values((
                relationships::follower_id.eq(follower_id),
                relationships::following_id.eq(following_id),
            ))
            .execute(conn)
            .and_then(|_| {
                diesel::update(users::table.find(follower_id))
                    .set(users::following_count.eq(users::following_count + 1))
                    .execute(conn)?;
                diesel::update(users::table.find(following_id))
                    .set(users::followers_count.eq(users::followers_count + 1))
                    .execute(conn)?;
                Ok(())
            })
    })
}
pub fn unfollow_user(
    follower_id: i32,
    following_id: i32,
    conn: &mut PgConnection,
) -> QueryResult<()> {
    conn.transaction(|conn| {
        diesel::delete(relationships::table)
            .filter(
                relationships::follower_id
                    .eq(follower_id)
                    .and(relationships::following_id.eq(following_id)),
            )
            .execute(conn)
            .and_then(|_| {
                diesel::update(users::table.find(follower_id))
                    .set(users::following_count.eq(users::following_count - 1))
                    .execute(conn)?;
                diesel::update(users::table.find(following_id))
                    .set(users::followers_count.eq(users::followers_count - 1))
                    .execute(conn)?;
                Ok(())
            })
    })
}
pub fn relation_exist(
    follower_id: i32,
    following_id: i32,
    conn: &mut PgConnection,
) -> QueryResult<bool> {
    diesel::select(diesel::dsl::exists(
        relationships::table.filter(
            relationships::follower_id
                .eq(follower_id)
                .and(relationships::following_id.eq(following_id)),
        ),
    ))
    .get_result::<bool>(conn)
}
