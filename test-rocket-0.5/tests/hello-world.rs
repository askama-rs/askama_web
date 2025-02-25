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

#[get("/")]
async fn hello<'r>() -> impl Responder<'r, 'static> {
    HelloTemplate { name: &"world" }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello])
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
}
