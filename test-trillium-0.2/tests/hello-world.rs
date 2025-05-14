use std::fmt::Display;

use askama::Template;
use askama_web::WebTemplate;
use trillium_testing::prelude::*;

#[derive(Template, WebTemplate)]
#[template(path = "hello.html")]
struct HelloWorld;

#[derive(Template, WebTemplate)]
#[template(path = "fail.html")]
struct FailWorld;

#[test]
fn hello_trillium_0_2() {
    assert_response!(
        get("/").on(&HelloWorld),
        Status::Ok,
        "Hello, world!",
        "content-type" => "text/html; charset=utf-8",
    );
}

#[test]
fn fail_trillium_0_2() {
    assert_response!(
        get("/").on(&FailWorld),
        Status::InternalServerError,
        "INTERNAL SERVER ERROR",
        "content-type" => "text/plain; charset=utf-8",
    );
}

#[derive(Debug, Clone, Copy)]
struct Fail;

impl Display for Fail {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Err(std::fmt::Error)
    }
}
