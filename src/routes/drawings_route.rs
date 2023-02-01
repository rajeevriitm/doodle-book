use crate::model::drawing::{Drawing, NewDrawing};
use crate::Db;
use rocket::form::Form;
use rocket::request::FlashMessage;
use rocket::response::{Flash, Redirect};
use rocket_dyn_templates::{context, Template};

#[get("/")]
pub fn home(flash: Option<FlashMessage<'_>>) -> Template {
    let flash = flash.map(FlashMessage::into_inner);
    Template::render("home", context! {name: "Rajeev", flash})
}
#[post("/create", data = "<drawing>")]
pub async fn save_drawing(drawing: Form<NewDrawing>, db: Db) -> Result<Flash<Redirect>, String> {
    // let drawing = drawing_form.value.as_ref().unwrap().clone();
    db.run(move |conn| drawing.save_to_db(conn))
        .await
        .map(|_out| Flash::success(Redirect::to(uri!(home)), "Successfully drawn"))
        .map_err(|_error| String::from("database error "))
}
