use std::fmt::Display;

use askama::Template;
use askama_web::WebTemplate;
use salvo::http::StatusCode;
use salvo::test::{ResponseExt, TestClient};
use salvo::{Router, Scribe, handler};

#[derive(Template, WebTemplate)]
#[template(path = "hello.html")]
struct HelloTemplate<'a, T>
where
    T: Display,
{
    name: &'a T,
}

#[handler]
async fn hello() -> impl Scribe {
    HelloTemplate { name: &"world" }
}

#[tokio::test]
async fn hello_salvo_0_77() {
    let router = Router::new().push(Router::with_path("/").get(hello));
    let mut resp = TestClient::get("http://127.0.0.1:0/").send(router).await;
    assert_eq!(resp.status_code, Some(StatusCode::OK));
    assert_eq!(
        resp.headers
            .get("content-type")
            .and_then(|h| h.to_str().ok()),
        Some("text/html; charset=utf-8")
    );
    assert_eq!(
        resp.take_string().await.as_deref().ok(),
        Some("Hello, world!")
    );
}
