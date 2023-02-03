#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
use rocket::fs::{relative, FileServer};
mod model;
mod routes;
mod schema;
use rocket::serde::Deserialize;
use rocket::{fairing::AdHoc, State};
use rocket_dyn_templates::Template;
use rocket_sync_db_pools::database;
use routes::drawings_route::{home, save_drawing};
use routes::users_route::{create_user, signup};
#[database("doodles")]
pub struct Db(diesel::PgConnection);
#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Configuration {
    password_salt: String,
}
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![home, save_drawing])
        .mount("/users", routes![signup, create_user])
        .attach(Template::fairing())
        .attach(Db::fairing())
        .attach(AdHoc::config::<Configuration>())
        .mount("/static", FileServer::from(relative!("assets/dist/")))
}
