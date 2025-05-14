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
async fn hello_salvo_0_76() {
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

#[handler]
async fn fail() -> impl Scribe {
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
async fn fail_salvo_0_76() {
    let router = Router::new().push(Router::with_path("/").get(fail));
    let mut resp = TestClient::get("http://127.0.0.1:0/").send(router).await;
    assert_eq!(resp.status_code, Some(StatusCode::INTERNAL_SERVER_ERROR));
    assert_eq!(
        resp.headers
            .get("content-type")
            .and_then(|h| h.to_str().ok()),
        Some("text/plain; charset=utf-8")
    );
    assert_eq!(
        resp.take_string().await.as_deref().ok(),
        Some("INTERNAL SERVER ERROR")
    );
}
