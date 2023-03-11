use crate::model::relationship::RelationshipForm;
use rocket::form::Form;
#[post("/folllow", data = "<following>")]
pub fn follow(following: Form<RelationshipForm>) {}
