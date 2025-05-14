use std::fmt::Display;

use askama::Template;
use askama_web::WebTemplate;
use rocket::http::Status;
use rocket::local::asynchronous::Client;
use rocket::response::Responder;
use rocket::{async_test, get, launch, routes, uri};

#[derive(Template, WebTemplate)]
#[template(path = "hello.html")]
struct HelloTemplate<'a, T>
where
    T: Display,
{
    name: &'a T,
}

#[get("/hello")]
async fn hello<'r>() -> impl Responder<'r, 'static> {
    HelloTemplate { name: &"world" }
}

#[get("/fail")]
async fn fail<'r>() -> impl Responder<'r, 'static> {
    HelloTemplate { name: &Fail }
}

#[derive(Debug, Clone, Copy)]
struct Fail;

impl Display for Fail {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Err(std::fmt::Error)
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello, fail])
}

#[async_test]
async fn hello_rocket_0_5() {
    let client = Client::tracked(rocket()).await.unwrap();

    let resp = client.get(uri!(hello)).dispatch().await;
    assert_eq!(resp.status(), Status::Ok);
    assert_eq!(
        resp.content_type().unwrap().to_string(),
        "text/html; charset=utf-8"
    );
    assert_eq!(resp.into_string().await.unwrap(), "Hello, world!");

    let resp = client.get(uri!(fail)).dispatch().await;
    assert_eq!(resp.status(), Status::InternalServerError);
    assert_eq!(
        resp.content_type().unwrap().to_string(),
        "text/plain; charset=utf-8"
    );
    assert_eq!(resp.into_string().await.unwrap(), "INTERNAL SERVER ERROR");
}
