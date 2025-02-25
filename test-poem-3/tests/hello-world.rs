use std::fmt::Display;

use askama::Template;
use askama_web::WebTemplate;
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
