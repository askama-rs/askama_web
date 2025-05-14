use std::fmt::Display;

use askama::Template;
use askama_web::WebTemplate;
use viz::{IntoResponse, Request, Result, Router, StatusCode, get};
use viz_test::TestServer;

#[derive(Template, WebTemplate)]
#[template(path = "hello.html")]
struct HelloTemplate<'a, T>
where
    T: Display,
{
    name: &'a T,
}

async fn hello(_: Request) -> Result<impl IntoResponse> {
    Ok(HelloTemplate { name: &"world" })
}

#[tokio::test]
async fn hello_viz_0_10() {
    let router = Router::new().route("/", get(hello));
    let server = TestServer::new(router).await.unwrap();
    let resp = server.get("/").send().await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    assert_eq!(
        resp.headers().get("Content-Type").unwrap(),
        "text/html; charset=utf-8",
    );
    assert_eq!(resp.bytes().await.as_deref().unwrap(), b"Hello, world!");
}

async fn fail(_: Request) -> Result<impl IntoResponse> {
    Ok(HelloTemplate { name: &Fail })
}

#[derive(Debug, Clone, Copy)]
struct Fail;

impl Display for Fail {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Err(std::fmt::Error)
    }
}

#[tokio::test]
async fn fail_viz_0_10() {
    let router = Router::new().route("/", get(fail));
    let server = TestServer::new(router).await.unwrap();
    let resp = server.get("/").send().await.unwrap();
    assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
    assert_eq!(
        resp.headers().get("Content-Type").unwrap(),
        "text/plain; charset=utf-8",
    );
    assert_eq!(
        resp.bytes().await.as_deref().unwrap(),
        b"INTERNAL SERVER ERROR"
    );
}
