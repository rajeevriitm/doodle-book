// use diesel::prelude::Table;
use diesel::query_dsl::methods::{LimitDsl, OffsetDsl};
use rocket::{
    request::{FromRequest, Outcome},
    Request,
};

// use crate::diesel::QueryDsl;
pub trait Paginator {
    const PER_PAGE: i64 = 10;
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
        let offset = (page - 1) * Self::PER_PAGE;
        self.limit(Self::PER_PAGE).offset(offset)
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
