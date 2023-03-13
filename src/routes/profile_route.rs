// use crate::model::{drawing, user};
use crate::model::{drawing::Drawing, relationship::*, user::User};
use crate::services::{AuthInfo, Page};
// use crate::schema::drawings;
use crate::Db;
use rocket::http::{Cookie, CookieJar};
use rocket::request::FlashMessage;
use rocket::response::Redirect;
use rocket_dyn_templates::{context, Template};
#[get("/?<page>", rank = 2)]
pub async fn unauth_home(flash: Option<FlashMessage<'_>>, db: Db, page: Option<i64>) -> Template {
    let page = page.unwrap_or(1);
    let flash = flash.map(FlashMessage::into_inner);
    // let flash = Some(("error", "created accoutn"));
    let user_drawings = db
        .run(move |conn| {
            let admin = User::find_user_with_name("Doodler", conn)?;
            let drawings = Drawing::user_drawings(&admin, page, conn).unwrap_or(vec![]);
            let user_drawings = create_user_list(&admin, drawings);
            Ok::<_, diesel::result::Error>(user_drawings)
        })
        .await
        .unwrap();
    let url = uri!(unauth_home(page = _)).to_string();
    let page = Page::new(page, &user_drawings, url);
    Template::render("unauth_home", context! {user_drawings, flash,page})
}
#[get("/?<page>", rank = 1)]
pub async fn auth_home(
    flash: Option<FlashMessage<'_>>,
    auth: AuthInfo,
    page: Option<i64>,
    db: Db,
    cookie_jar: &CookieJar<'_>,
) -> Result<Template, Redirect> {
    let flash = flash.map(FlashMessage::into_inner);
    let page = page.unwrap_or(1);
    let result = db
        .run(move |conn| {
            let current_user = User::find(auth.user_id, conn)?;
            let drawings = Drawing::home(&current_user, page, conn).unwrap_or(vec![]);
            Ok::<_, diesel::result::Error>((current_user, drawings))
        })
        .await;
    result
        .map(|(user, user_drawings)| {
            let url = uri!(auth_home(page = _)).to_string();
            let page = Page::new(page, &user_drawings, url);
            Template::render(
                "auth_home",
                context! {current_user_id: user.id,user,user_drawings,flash,page},
            )
        })
        .map_err(|_| {
            cookie_jar.remove_private(Cookie::named("user_id"));
            Redirect::to("/")
        })
}
#[get("/user/<id>?<page>")]
pub async fn user_profile(
    db: Db,
    id: i32,
    page: Option<i64>,
    auth: Option<AuthInfo>,
    flash: Option<FlashMessage<'_>>,
) -> Result<Template, Redirect> {
    let page = page.unwrap_or(1);
    let flash = flash.map(FlashMessage::into_inner);
    let current_user_id = auth.map(|auth| auth.user_id);
    let result = db
        .run(move |conn| {
            let user = User::find(id, conn)?;
            let drawings = Drawing::user_drawings(&user, page, conn).unwrap_or(vec![]);
            let user_drawings = create_user_list(&user, drawings);
            let show_unfollow_btn = current_user_id
                .map(|current_user_id| relation_exist(current_user_id, user.id, conn))
                .transpose()?
                .map(|following| choose_action_and_url(following));
            Ok::<_, diesel::result::Error>((user, user_drawings, show_unfollow_btn))
        })
        .await;
    result
        .map(|(user, user_drawings, show_unfollow_btn)| {
            let url = uri!(user_profile(id = id, page = _)).to_string();
            let page = Page::new(page, &user_drawings, url);
            Template::render(
                "user",
                context! {flash,user,user_drawings,current_user_id,page,show_unfollow_btn},
            )
        })
        .map_err(|_| Redirect::to("/"))
}
fn choose_action_and_url(following: bool) -> (bool, String) {
    let url = if following {
        uri!(
            "/relationship",
            crate::routes::relationships_route::unfollow()
        )
    } else {
        uri!(
            "/relationship",
            crate::routes::relationships_route::follow()
        )
    };
    (following, url.to_string())
}
fn create_user_list(user: &User, drawings: Vec<Drawing>) -> Vec<(User, Drawing)> {
    drawings
        .into_iter()
        .map(|drawing| (user.clone(), drawing))
        .collect::<Vec<(User, Drawing)>>()
}
