#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
use rocket::form::{error, Contextual, Form};
use rocket::fs::{relative, FileServer};
mod model;
mod schema;
use model::{Db, Drawing, NewDrawing};
use rocket::response::Redirect;
use rocket_dyn_templates::{context, Template};

// use rocket::serde::json;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![home, save_drawing])
        .attach(Template::fairing())
        .attach(Db::fairing())
        .mount("/static", FileServer::from(relative!("assets/dist/")))
}
#[get("/")]
fn home() -> Template {
    Template::render("home", context! {name: "Rajeev"})
}
#[post("/create", data = "<drawing_form>")]
async fn save_drawing(drawing_form: Form<Contextual<'_, NewDrawing>>, db: Db) -> Redirect {
    let drawing = drawing_form.value.as_ref().unwrap().clone();
    let db_exec = db
        .run(move |conn| drawing.save_to_db(conn))
        .await
        .map_err(|_error| String::from("database error "));
    println!("{:?}", drawing_form.context);
    // Ok(String::from("fff"))
    Redirect::to(uri!(home))
    // let draw = format!("{:?}", val);
    // println!("{:?}", drawing_form.points);
}
// struct Drawing {
//     points: Vec<Vec<i32>>,
//     // points: Vec<Vec<(i32, i32)>>,
// }
