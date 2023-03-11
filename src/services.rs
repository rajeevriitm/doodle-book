const PER_PAGE: i64 = 8;
use diesel::query_dsl::methods::{LimitDsl, OffsetDsl};
use rocket::{
    request::{FromRequest, Outcome},
    Request,
};
use serde::Serialize;
// use rocket::http::uri::Origin;
// use crate::routes::profile_route;
// use crate::diesel::QueryDsl;
pub trait Paginator {
    type Output;
    fn paginate(self, page: i64) -> Self::Output;
}
type OffsetDslOutput<T> = <T as OffsetDsl>::Output;
type LimitDslOutput<T> = <T as LimitDsl>::Output;
impl<T> Paginator for T
where
    Self: LimitDsl + Sized,
    <Self as LimitDsl>::Output: OffsetDsl,
{
    type Output = OffsetDslOutput<LimitDslOutput<T>>;
    fn paginate(self, page: i64) -> Self::Output {
        let offset = (page - 1) * PER_PAGE;
        self.limit(PER_PAGE).offset(offset)
    }
}
#[derive(Serialize)]
pub struct Page {
    current: i64,
    next_page: Option<String>,
    prev_page: Option<String>,
}
impl Page {
    pub fn new<T>(current: i64, list: &Vec<T>, url: String) -> Self {
        let next_page =
            (list.len() == PER_PAGE as usize).then(|| Page::create_url(url.clone(), current + 1));
        let prev_page = (current > 1).then(|| Page::create_url(url, current - 1));
        Page {
            current,
            next_page,
            prev_page,
        }
    }
    fn create_url(url: String, page: i64) -> String {
        format!("{}?page={}", url, page)
    }
}
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
    pub user_id: i32,
}

//users::table.filter(users::id.name("raj")).paginate(1)
