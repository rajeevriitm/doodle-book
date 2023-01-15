#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![home])
}
#[get("/home")]
fn home() {
    println!("Hello, world!");
}
