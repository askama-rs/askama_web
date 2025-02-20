use std::fmt::Display;

use actix_web::http::header::CONTENT_TYPE;
use actix_web::web;
use askama::Template;
use askama_web::WebTemplate;
use bytes::Bytes;

#[derive(Template, WebTemplate)]
#[template(path = "hello.html")]
struct HelloTemplate<'a, T>
where
    T: Display,
{
    name: &'a T,
}

#[actix_rt::test]
async fn hello_actix_web_4() {
    let srv = actix_test::start(|| {
        actix_web::App::new()
            .service(web::resource("/").to(|| async { HelloTemplate { name: &"world" } }))
    });

    let request = srv.get("/");
    let mut response = request.send().await.unwrap();
    assert!(response.status().is_success());
    assert_eq!(
        response.headers().get(CONTENT_TYPE).unwrap(),
        "text/html; charset=utf-8"
    );
    let bytes = response.body().await.unwrap();
    assert_eq!(bytes, Bytes::from_static(b"Hello, world!"));
}
