use crate::model::user::SignupForm;
use crate::Configuration;
use crate::Db;
use rocket::form::{Contextual, Form};
use rocket::request::FlashMessage;
use rocket::response::{Flash, Redirect};
use rocket::State;
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
    config: &State<Configuration>,
) -> Result<Flash<Redirect>, Template> {
    match &user_form.value {
        Some(user) => {
            let mut user = user.clone();
            match user.hash_password(&config.password_salt) {
                Ok(hash) => user.password = hash,
                Err(err) => {
                    return Err(Template::render("signup", context! {flash: ("error",err)}))
                }
            }
            dbg!(&user);
            db.run(move |conn| {
                user.save_to_db(conn)
                    .map(|_| Flash::success(Redirect::to("/"), "Successfully Signed Up"))
                    .map_err(|err| {
                        dbg!(err);
                        Template::render(
                            "signup",
                            context! {flash: ("error","Error occured while saving")},
                        )
                    })
            })
            .await
        }
        None => {
            let flash = user_form
                .context
                .errors()
                .next()
                .map(|x| format!("{}: {}", x.name.as_ref().unwrap(), x))
                .unwrap_or("Error occured in form".to_string());
            Err(Template::render(
                "signup",
                context! {flash: ("error",flash)},
            ))
        } // dbg!(user_form);
    } // todo!();
}
