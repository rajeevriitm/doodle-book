pub mod drawings_route;
pub mod profile_route;
pub mod users_route;
use rocket::{
    request::{FromRequest, Outcome},
    Request,
};
#[rocket::async_trait]
impl<'a> FromRequest<'a> for AuthInfo {
    type Error = ();
    async fn from_request(req: &'a Request<'_>) -> Outcome<Self, Self::Error> {
        let cookies = req.cookies();
        let id = cookies
            .get_private("user_id")
            .and_then(|cookie| cookie.value().parse().ok());
        match id {
            Some(user_id) => Outcome::Success(AuthInfo { user_id }),
            None => Outcome::Forward(()),
        }
    }
}
pub struct AuthInfo {
    user_id: i32,
}
