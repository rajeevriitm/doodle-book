use crate::model::user::{User, UserForm, UserUpdateForm};
use crate::routes::profile_route;
use crate::routes::AuthInfo;
use crate::Configuration;
use crate::Db;
use rocket::form::{Contextual, Form};
use rocket::http::{Cookie, CookieJar};
use rocket::request::FlashMessage;
use rocket::response::{Flash, Redirect};
use rocket::State;
use rocket_dyn_templates::{context, Template};
#[get("/edit")]
pub async fn edit_profile(auth: AuthInfo, db: Db) -> Result<Template, Redirect> {
    let user = db
        .run(move |conn| User::find(auth.user_id, conn).or(Err(Redirect::to("/"))))
        .await?;
    Ok(Template::render(
        "user_edit",
        context![current_user_id: user.id,user],
    ))
}
#[put("/update", data = "<form>")]
pub async fn update_user(
    db: Db,
    auth: AuthInfo,
    mut form: Form<UserUpdateForm>,
) -> Flash<Redirect> {
    // dbg!(&form);
    if let Some(points) = &form.profile_pic {
        let points = serde_json::from_str::<Vec<Vec<[i32; 2]>>>(points).unwrap();
        if points.len() == 0 {
            form.profile_pic = None;
            form.profile_pic_width = None;
        }
    }
    let redirect = Redirect::to(uri!(profile_route::user_profile(id = auth.user_id)));
    let result = db
        .run(move |conn| form.update_user(auth.user_id, conn))
        .await;
    if result.is_ok() {
        Flash::success(redirect, "Succesfully updated")
    } else {
        Flash::error(redirect, "Error occured")
    }
}
#[get("/signup", rank = 1)]
pub fn authenticated_signup(_user: AuthInfo) -> Redirect {
    Redirect::to("/")
}
#[get("/signin", rank = 1)]
pub fn authenticated_signin(_user: AuthInfo) -> Redirect {
    Redirect::to("/")
}
#[get("/signup", rank = 2)]
pub fn signup(flash: Option<FlashMessage<'_>>) -> Template {
    let flash = flash.map(FlashMessage::into_inner);
    Template::render("signup", context! {flash})
}
#[get("/signin", rank = 2)]
pub fn signin(flash: Option<FlashMessage<'_>>) -> Template {
    let flash = flash.map(FlashMessage::into_inner);
    Template::render("signin", context! {flash})
}
#[post("/session", data = "<signin_form>")]
pub async fn create_session(
    signin_form: Form<Contextual<'_, UserForm>>,
    db: Db,
    cookie_jar: &CookieJar<'_>,
) -> Result<Redirect, Template> {
    match &signin_form.value {
        Some(user_form) => {
            let email = user_form.email.clone();
            // let password = user_form.password.clone();
            let user_result = db
                .run(move |conn| User::find_user_with_email(email, conn))
                .await;
            let user =
                user_result.map_err(|_err| render_template("signin", "Email address incorrect"))?;
            if user.verify_password(&user_form.password).is_ok() {
                let cookie = Cookie::new("user_id", user.id.to_string());
                cookie_jar.add_private(cookie);
                Ok(Redirect::to("/"))
            } else {
                return Err(render_template("signin", "Incorrect password"));
            }
        }
        None => Err(Template::render(
            "signin",
            context! {flash: ("error","Invalid credentials")},
        )),
    }
    // dbg!(signin_form);
    // todo!()
}
#[post("/create", data = "<user_form>")]
pub async fn create_user(
    user_form: Form<Contextual<'_, UserForm>>,
    db: Db,
    config: &State<Configuration>,
) -> Result<Flash<Redirect>, Template> {
    match &user_form.value {
        Some(user) => {
            let mut user = user.clone();
            match user.hash_password(&config.password_salt) {
                Ok(hash) => user.password = hash,
                Err(err) => {
                    return Err(render_template("signup", err));
                }
            }
            dbg!(&user);
            db.run(move |conn| {
                user.save_to_db(conn)
                    .map(|_| {
                        Flash::success(
                            Redirect::to(uri!("/users", signin)),
                            "Successfully Signed Up",
                        )
                    })
                    .map_err(|err| render_template("signup", "Unable to save due to error occured"))
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
            Err(render_template("signup", &flash))
        }
    }
}
#[post("/signout")]
pub fn signout(cookie: &CookieJar<'_>) -> Flash<Redirect> {
    cookie.remove_private(Cookie::named("user_id"));
    Flash::success(
        Redirect::to(uri!("/users", signin)),
        "Logged out succesfully!",
    )
}
fn render_template(template: &'static str, err: &str) -> Template {
    Template::render(template, context! { flash: ("error",err)})
}
