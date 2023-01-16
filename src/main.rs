#[macro_use]
extern crate rocket;
use rocket::fs::{relative, FileServer};
use rocket_dyn_templates::{context, Template};
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![home])
        .attach(Template::fairing())
        .mount("/static", FileServer::from(relative!("/templates/dist/")))
}
#[get("/")]
fn home() -> Template {
    Template::render("home", context! {name: "Rajeev"})
}
