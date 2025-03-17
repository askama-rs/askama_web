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
