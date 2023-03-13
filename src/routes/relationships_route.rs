use crate::model::relationship::*;
use crate::services::{AuthInfo, ReferrerUrl};
use crate::Db;
use rocket::form::Form;
use rocket::response::{Flash, Redirect};
#[post("/follow", data = "<following>")]
pub async fn follow(
    following: Form<RelationshipForm>,
    auth: AuthInfo,
    db: Db,
    referer: ReferrerUrl,
) -> Flash<Redirect> {
    let referer_url = referer.0;
    let result = db
        .run(move |conn| {
            follow_user(auth.user_id, following.following_id, conn)?;
            Ok::<_, diesel::result::Error>(())
        })
        .await;
    match result {
        Ok(_) => Flash::success(Redirect::to(referer_url), "Successfully followed"),
        Err(_) => Flash::error(Redirect::to(referer_url), "Error occured"),
    }
}
#[delete("/unfollow", data = "<following>")]
pub async fn unfollow(
    following: Form<RelationshipForm>,
    auth: AuthInfo,
    db: Db,
    referer: ReferrerUrl,
) -> Flash<Redirect> {
    let referer_url = referer.0;
    let result = db
        .run(move |conn| {
            unfollow_user(auth.user_id, following.following_id, conn)?;
            Ok::<_, diesel::result::Error>(())
        })
        .await;
    match result {
        Ok(_) => Flash::success(Redirect::to(referer_url), "Successfully unfollowed"),
        Err(_) => Flash::error(Redirect::to(referer_url), "Error occured"),
    }
}
