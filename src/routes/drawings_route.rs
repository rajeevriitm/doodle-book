use crate::model::drawing::{Drawing, DrawingForm};
use crate::routes::AuthInfo;
use crate::Db;
use rocket::form::Form;
use rocket::response::{Flash, Redirect};
use rocket::State;
use rocket_dyn_templates::{context, Template};
#[post("/create", data = "<drawing>")]
pub async fn save_drawing(
    drawing: Form<DrawingForm<'_>>,
    db: Db,
    auth: AuthInfo,
) -> Result<Flash<Redirect>, String> {
    let drawing = drawing.get_new_drawing(auth.user_id);
    db.run(move |conn| drawing.save_to_db(conn))
        .await
        .map(|_out| Flash::success(Redirect::to("/"), "Successfully drawn"))
        .map_err(|_error| String::from("database error "))
}
