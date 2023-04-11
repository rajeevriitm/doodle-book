#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
mod services;
#[cfg(test)]
mod test;
use rocket::fs::{relative, FileServer};
mod model;
mod routes;
mod schema;
use rocket::fairing::AdHoc;
use rocket::serde::Deserialize;
use rocket_dyn_templates::Template;
use rocket_sync_db_pools::database;
use routes::drawings_route::{delete_drawing, save_drawing};
use routes::profile_route::{auth_home, unauth_home, user_profile};
use routes::relationships_route::{follow, unfollow};
use routes::users_route::{
    authenticated_signin, authenticated_signup, create_session, create_user, edit_profile, signin,
    signout, signup, update_user,
};
#[database("doodles")]
pub struct Db(diesel::PgConnection);
#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Configuration {
    password_salt: String,
}
#[launch]
fn rocket() -> _ {
    for (key, value) in std::env::vars() {
        println!("{}={}", key, value);
    }
    rocket::build()
        .mount(
            "/",
            routes![
                unauth_home,
                save_drawing,
                user_profile,
                auth_home,
                delete_drawing
            ],
        )
        .mount(
            "/users",
            routes![
                signup,
                create_user,
                signin,
                create_session,
                authenticated_signin,
                authenticated_signup,
                signout,
                edit_profile,
                update_user
            ],
        )
        .mount("/relationship", routes![follow, unfollow])
        .attach(Template::fairing())
        .attach(Db::fairing())
        .attach(AdHoc::config::<Configuration>())
        .mount("/static", FileServer::from(relative!("assets/static/")))
}
