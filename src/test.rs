use super::model::{drawing::Drawing, user::User};
use super::routes::{drawings_route, users_route};
use rocket::http::{ContentType, Status};
use rocket::local::asynchronous;
use std::sync::Mutex;
static ITERATION: Mutex<usize> = Mutex::new(0);
const TEST_USER: &str = "username=test&email=test@email&password=iitm";
#[test]
fn signup_return_ok() {
    let client = rocket::local::blocking::Client::tracked(super::rocket()).unwrap();
    let response = client.get("/users/signup").dispatch();
    assert_eq!(response.status(), rocket::http::Status::Ok);
}
#[test]
fn signin_return_ok() {
    let client = rocket::local::blocking::Client::tracked(super::rocket()).unwrap();
    let response = client.get("/users/signin").dispatch();
    assert_eq!(response.status(), rocket::http::Status::Ok);
}
#[rocket::async_test]
async fn test_unauth_home() {
    let client = async_init().await;
    let response = client.get("/").dispatch().await;
    assert_eq!(response.status(), rocket::http::Status::Ok);
    assert!(response.into_string().await.unwrap().contains("Welcome"));
}
#[rocket::async_test]
async fn signup_succeeds() {
    let client = async_init().await;
    let body = get_body();
    // dbg!(&body);
    let mut response = client_signup(&body, &client).await;
    assert_eq!(response.status(), Status::SeeOther);
    let url = response.headers().get_one("Location").unwrap();
    assert_eq!(url, &uri!("/users", users_route::signin));
    response = client_signup(&body, &client).await;
    assert_eq!(response.status(), Status::Ok);
    assert!(response.into_string().await.unwrap().contains("error"));
}
#[rocket::async_test]
async fn signin_succeeds() {
    let client = async_init().await;
    let body = get_body();
    client_signup(&body, &client).await;
    let mut response = client_signin(&body, &client).await;
    assert_eq!(response.status(), Status::SeeOther);
    // let url = response.headers().get_one("Location").unwrap();
    assert_eq!(response.headers().get_one("Location").unwrap(), "/");
    response = client_signin("email=rajeev@email&password=ijklm", &client).await;
    assert_eq!(response.status(), Status::Ok);
    // dbg!(response.into_string().await.unwrap());
    assert!(response.into_string().await.unwrap().contains("incorrect"));
    let response = client.get("/users/signin").dispatch().await;
    assert_eq!(response.status(), Status::SeeOther);
    assert_eq!(response.headers().get_one("Location").unwrap(), "/");
}
#[rocket::async_test]
async fn auth_home() {
    let client = async_init().await;
    client_signin(TEST_USER, &client).await;
    let response = client
        .post(uri!(drawings_route::save_drawing))
        .header(ContentType::Form)
        .body("points=[[[1,2]]]&width=50")
        .dispatch()
        .await;
    assert_eq!(response.status(), Status::SeeOther);
    assert_eq!(response.headers().get_one("Location").unwrap(), "/");
    let response = client.get("/").dispatch().await;
    assert!(response.into_string().await.unwrap().contains("[[[1,2]]]"));
}
async fn async_init() -> asynchronous::Client {
    let rocket = super::rocket();
    let client = rocket::local::asynchronous::Client::tracked(rocket)
        .await
        .unwrap();
    let mut lock = ITERATION.lock().unwrap();
    if *lock == 0 {
        delete_users(&client).await;
        client_signup(TEST_USER, &client).await;
    }
    *lock += 1;
    client
}
fn get_body() -> String {
    format!(
        "username=rajeev&password=iitm&email=raj{}@email",
        ITERATION.lock().unwrap()
    )
}
async fn client_signin<'a>(
    body: &str,
    client: &'a asynchronous::Client,
) -> asynchronous::LocalResponse<'a> {
    let signin_url = uri!("/users", users_route::create_session);
    client
        .post(signin_url)
        .body(body)
        .header(ContentType::Form)
        .dispatch()
        .await
}
async fn client_signup<'a>(
    body: &str,
    client: &'a asynchronous::Client,
) -> asynchronous::LocalResponse<'a> {
    let signup_url = uri!("/users", users_route::create_user());
    client
        .post(signup_url)
        .header(ContentType::Form)
        .body(body)
        .dispatch()
        .await
}
async fn delete_users(client: &asynchronous::Client) {
    let db = super::Db::get_one(client.rocket()).await.unwrap();
    db.run(|conn| {
        Drawing::delete_all(conn);
        User::delete_all(conn);
    })
    .await;
}
// fn init_test() {
//     dotenv::from_filename("test.env").unwrap();
// }
