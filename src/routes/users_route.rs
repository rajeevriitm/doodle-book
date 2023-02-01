use crate::model::user::SignupForm;
use crate::Db;
use rocket::form::{Contextual, Form};
use rocket::request::FlashMessage;
use rocket::response::{Flash, Redirect};

// use rocket_dyn_templates::Template;

// use rocket::
use rocket_dyn_templates::{context, Template};

// use serde_json::json;
#[get("/signup")]
pub fn signup() -> Template {
    Template::render("signup", context! {})
}
#[post("/create", data = "<user_form>")]
pub async fn create_user(
    user_form: Form<Contextual<'_, SignupForm>>,
    db: Db,
) -> Result<Flash<Redirect>, Template> {
    match &user_form.value {
        Some(user) => {
            let user = user.clone();
            dbg!(&user);
            db.run(move |conn| {
                user.save_to_db(conn)
                    .map(|_| {
                        Flash::success(
                            Redirect::to(uri!("/users", signup)),
                            "Successfully Signedup",
                        )
                    })
                    .map_err(|err| {
                        dbg!(err);
                        Template::render("signup", context! {})
                    })
            })
            .await
        }
        None => Err(Template::render("signup", context! {})), // dbg!(user_form);
    } // todo!();
}
