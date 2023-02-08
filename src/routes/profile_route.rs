use crate::model::user;
use crate::model::{drawing::Drawing, user::User};
use crate::routes::AuthInfo;
use crate::Db;
use rocket::http::{Cookie, CookieJar};
use rocket::request::FlashMessage;
use rocket::response::{Flash, Redirect};
use rocket_dyn_templates::{context, Template};
#[get("/", rank = 2)]
pub fn home(flash: Option<FlashMessage<'_>>) -> Template {
    // dbg!(config);
    let flash = flash.map(FlashMessage::into_inner);
    Template::render("home", context! {name: "Rajeev", flash})
}
#[get("/", rank = 1)]
pub async fn auth_home(
    flash: Option<FlashMessage<'_>>,
    auth: AuthInfo,
    db: Db,
    cookie_jar: &CookieJar<'_>,
) -> Result<Template, Redirect> {
    let result = db
        .run(move |conn| {
            let user = User::find(auth.user_id, conn)?;
            let drawings = Drawing::user_drawings(&user, conn).unwrap_or(vec![]);
            Ok::<_, diesel::result::Error>((user, drawings))
        })
        .await;
    result
        .map(|(user, drawings)| Template::render("home", context! {user,drawings}))
        .map_err(|_| {
            cookie_jar.remove_private(Cookie::named("user_id"));
            Redirect::to("/")
        })
}
#[get("/user/<id>")]
pub async fn user_profile(db: Db, id: i32) -> Result<Template, Redirect> {
    let result = db
        .run(move |conn| {
            let user = User::find(id, conn)?;
            let drawings = Drawing::user_drawings(&user, conn).unwrap_or(vec![]);
            Ok::<_, diesel::result::Error>((user, drawings))
        })
        .await;
    result
        .map(|(user, drawings)| Template::render("home", context! {user,drawings}))
        .map_err(|_| Redirect::to("/"))
}
