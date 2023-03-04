use crate::model::{drawing, user};
use crate::model::{drawing::Drawing, user::User};
use crate::routes::AuthInfo;
use crate::schema::drawings;
use crate::Db;
use rocket::http::{Cookie, CookieJar};
use rocket::request::FlashMessage;
use rocket::response::{Flash, Redirect};
use rocket_dyn_templates::{context, Template};
#[get("/", rank = 2)]
pub async fn unauth_home(flash: Option<FlashMessage<'_>>, db: Db) -> Template {
    let flash = flash.map(FlashMessage::into_inner);
    let user_drawings = db
        .run(move |conn| {
            // let config = User::count(conn);
            // dbg!(config);

            let admin = User::find_user_with_name("Doodler", conn)?;
            let drawings = Drawing::user_drawings(&admin, conn).unwrap_or(vec![]);
            let user_drawings = create_user_list(&admin, drawings);
            Ok::<_, diesel::result::Error>(user_drawings)
        })
        .await
        .unwrap();
    Template::render("unauth_home", context! {user_drawings, flash})
}
#[get("/", rank = 1)]
pub async fn auth_home(
    flash: Option<FlashMessage<'_>>,
    auth: AuthInfo,
    db: Db,
    cookie_jar: &CookieJar<'_>,
) -> Result<Template, Redirect> {
    let flash = flash.map(FlashMessage::into_inner);
    let result = db
        .run(move |conn| {
            let current_user = User::find(auth.user_id, conn)?;
            let drawings = Drawing::home(&current_user, conn).unwrap_or(vec![]);
            Ok::<_, diesel::result::Error>((current_user, drawings))
        })
        .await;
    result
        .map(|(user, user_drawings)| {
            Template::render(
                "auth_home",
                context! {current_user_id: user.id,user,user_drawings,flash},
            )
        })
        .map_err(|_| {
            cookie_jar.remove_private(Cookie::named("user_id"));
            Redirect::to("/")
        })
}
#[get("/user/<id>")]
pub async fn user_profile(
    db: Db,
    id: i32,
    auth: Option<AuthInfo>,
    flash: Option<FlashMessage<'_>>,
) -> Result<Template, Redirect> {
    let flash = flash.map(FlashMessage::into_inner);
    let current_user_id = auth.map(|auth| auth.user_id);
    let result = db
        .run(move |conn| {
            let user = User::find(id, conn)?;
            let drawings = Drawing::user_drawings(&user, conn).unwrap_or(vec![]);
            let user_drawings = create_user_list(&user, drawings);
            Ok::<_, diesel::result::Error>((user, user_drawings))
        })
        .await;
    result
        .map(|(user, user_drawings)| {
            Template::render("user", context! {flash,user,user_drawings,current_user_id})
        })
        .map_err(|_| Redirect::to("/"))
}
fn create_user_list(user: &User, drawings: Vec<Drawing>) -> Vec<(User, Drawing)> {
    drawings
        .into_iter()
        .map(|drawing| (user.clone(), drawing))
        .collect::<Vec<(User, Drawing)>>()
}
