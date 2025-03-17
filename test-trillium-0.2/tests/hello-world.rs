use askama::Template;
use askama_web::WebTemplate;
use trillium_testing::prelude::*;

#[derive(Template, WebTemplate)]
#[template(path = "hello.html")]
struct HelloWorld;

#[test]
fn hello_trillium_0_2() {
    assert_response!(
        get("/").on(&HelloWorld),
        Status::Ok,
        "Hello, world!",
        "content-type" => "text/html; charset=utf-8",
    );
}
