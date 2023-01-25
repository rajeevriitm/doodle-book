#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
use rocket::form::{error, Form};
use rocket::fs::{relative, FileServer};
mod model;
mod schema;
use model::{Db, Drawing, NewDrawing};
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
#[post("/create", data = "<drawing>")]
async fn save_drawing(drawing: Form<NewDrawing>, db: Db) -> Result<String, String> {
    // let res: Vec<String> = json::from_str(&drawing.points).unwrap();
    // let res: Vec<String> = json::from_str(&res);
    let res = serde_json::from_str::<Vec<Vec<(i32, i32)>>>(&drawing.points);
    let res = serde_json::from_str::<Vec<Vec<(i32)>>>("[[(1)]]");
    // let bool = res.unwrap().iter().flatten().count();
    println!("{:?}", &res);
    let val = db
        .run(move |conn| drawing.save_to_db(conn))
        .await
        .map_err(|_error| String::from("vann error "))?;
    let draw = format!("{:?}", val);
    Ok(draw)
    // println!("{:?}", drawing.points);
}
// struct Drawing {
//     points: Vec<Vec<i32>>,
//     // points: Vec<Vec<(i32, i32)>>,
// }
