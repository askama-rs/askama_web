use std::fmt::Display;

use askama::Template;
use askama_web::WebTemplateExt;
use axum::Router;
use axum::body::Body;
use axum::http::{Request, StatusCode};
use axum::response::IntoResponse;
use axum::routing::get;
use http_body_util::BodyExt;
use tower::util::ServiceExt;

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate<'a, T>
where
    T: Display,
{
    name: &'a T,
}

async fn hello() -> impl IntoResponse {
    HelloTemplate { name: &"world" }.as_web_template()
}

#[tokio::test]
async fn hello_axum_0_8() {
    let app = Router::new().route("/", get(hello));

    let res = app
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    let headers = res.headers();
    assert_eq!(headers["Content-Type"], "text/html; charset=utf-8");

    let body = res.into_body().collect().await.unwrap().to_bytes();
    assert_eq!(&body[..], b"Hello, world!");
}

async fn fail() -> impl IntoResponse {
    HelloTemplate { name: &Fail }.as_web_template()
}

#[derive(Debug, Clone, Copy)]
struct Fail;

impl Display for Fail {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Err(std::fmt::Error)
    }
}

#[tokio::test]
async fn fail_axum_0_8() {
    let app = Router::new().route("/", get(fail));

    let res = app
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::INTERNAL_SERVER_ERROR);

    let headers = res.headers();
    assert_eq!(headers["Content-Type"], "text/plain; charset=utf-8");

    let body = res.into_body().collect().await.unwrap().to_bytes();
    assert_eq!(&body[..], b"INTERNAL SERVER ERROR");
}
