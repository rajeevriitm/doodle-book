use crate::model::drawing::{Drawing, DrawingForm};
use crate::services::AuthInfo;
use crate::Db;
use rocket::form::Form;
// use rocket::request::Request;
use rocket::response::{Flash, Redirect};
// use rocket_dyn_templates::{context, Template};

#[post("/create", data = "<drawing>")]
pub async fn save_drawing(
    drawing: Form<DrawingForm<'_>>,
    db: Db,
    auth: AuthInfo,
) -> Result<Flash<Redirect>, String> {
    let drawing = drawing.get_new_drawing(auth.user_id);
    // dbg!(&drawing.);
    db.run(move |conn| drawing.save_to_db(conn))
        .await
        .map(|_out| Flash::success(Redirect::to("/"), "Successfully drawn"))
        .map_err(|_error| String::from("database error "))
}
#[delete("/delete_drawing/<id>")]
pub async fn delete_drawing(id: i32, db: Db, auth: AuthInfo) -> Flash<Redirect> {
    let result = db
        .run(move |conn| {
            Drawing::find_drawing(id, conn)
                .or(Err("Unknown drawing"))
                .and_then(|drawing| {
                    if auth.user_id == drawing.user_id {
                        Drawing::delete_drawing(id, conn)
                            .and(Ok("Successfully deleted"))
                            .or(Err("Error occured"))
                    } else {
                        Err("Unauthorised delete request")
                    }
                })
        })
        .await;
    match result {
        Ok(msg) => Flash::success(Redirect::to("/"), msg),
        Err(msg) => Flash::error(Redirect::to("/"), msg),
    }
}
