use std::fmt::Display;

use askama::Template;
use askama_web::WebTemplate;
use poem::http::StatusCode;
use poem::test::TestClient;
use poem::{IntoResponse, Route, get, handler};

#[derive(Template, WebTemplate)]
#[template(path = "hello.html")]
struct HelloTemplate<'a, T>
where
    T: Display,
{
    name: &'a T,
}

#[handler]
async fn hello() -> impl IntoResponse {
    HelloTemplate { name: &"world" }
}

#[tokio::test]
async fn hello_poem_3() {
    let app = Route::new().at("/", get(hello));
    let resp = TestClient::new(app).get("/").send().await;
    resp.assert_status_is_ok();
    resp.assert_content_type("text/html; charset=utf-8");
    resp.assert_text("Hello, world!").await;
}

#[handler]
async fn fail() -> impl IntoResponse {
    HelloTemplate { name: &Fail }
}

#[derive(Debug, Clone, Copy)]
struct Fail;

impl Display for Fail {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Err(std::fmt::Error)
    }
}

#[tokio::test]
async fn fail_poem_3() {
    let app = Route::new().at("/", get(fail));
    let resp = TestClient::new(app).get("/").send().await;
    resp.assert_status(StatusCode::INTERNAL_SERVER_ERROR);
    resp.assert_content_type("text/plain; charset=utf-8");
    resp.assert_text("INTERNAL SERVER ERROR").await;
}
