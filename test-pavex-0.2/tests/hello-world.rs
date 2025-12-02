use std::fmt::Display;

use askama::Template;
use askama_web::WebTemplate;
use pavex::IntoResponse;

#[derive(Template, WebTemplate)]
#[template(path = "hello.html")]
struct HelloTemplate<'a, T>
where
    T: Display,
{
    name: &'a T,
}

#[test]
fn hello_pavex_0_2() {
    let tmpl = HelloTemplate { name: &"world" };
    let response = tmpl.into_response();

    assert_eq!(response.status().as_u16(), 200);

    let content_type = response
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok());
    assert_eq!(content_type, Some("text/html; charset=utf-8"));
}

#[derive(Debug, Clone, Copy)]
struct Fail;

impl Display for Fail {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Err(std::fmt::Error)
    }
}

#[test]
fn fail_pavex_0_2() {
    let tmpl = HelloTemplate { name: &Fail };
    let response = tmpl.into_response();

    assert_eq!(response.status().as_u16(), 500);
}
