use std::fmt::Display;

use askama::Template;
use askama_web::WebTemplate;
use warp::Filter;
use warp::reply::Reply;

#[derive(Template, WebTemplate)]
#[template(path = "hello.html")]
struct HelloTemplate<'a, T>
where
    T: Display,
{
    name: &'a T,
}

fn hello() -> impl Reply {
    HelloTemplate { name: &"world" }
}

#[tokio::test]
async fn hello_warp_0_3() {
    let routes = warp::get().and(warp::path!().map(hello));
    let resp = warp::test::request().reply(&routes).await;
    assert_eq!(resp.status(), 200);
    assert_eq!(
        resp.headers().get("content-type").unwrap(),
        "text/html; charset=utf-8"
    );
    assert_eq!(&**resp.body(), b"Hello, world!");
}

fn fail() -> impl Reply {
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
async fn fail_warp_0_3() {
    let routes = warp::get().and(warp::path!().map(fail));
    let resp = warp::test::request().reply(&routes).await;
    assert_eq!(resp.status(), 500);
    assert_eq!(
        resp.headers().get("content-type").unwrap(),
        "text/plain; charset=utf-8"
    );
    assert_eq!(&**resp.body(), b"INTERNAL SERVER ERROR");
}
