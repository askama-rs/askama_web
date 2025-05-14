use std::fmt::Display;

use askama::Template;
use askama_web::WebTemplate;
use cot::config::ProjectConfig;
use cot::project::RegisterAppsContext;
use cot::response::IntoResponse;
use cot::router::{Route, Router};
use cot::test::Client;
use cot::{App, AppBuilder, Project};

#[derive(Template, WebTemplate)]
#[template(path = "hello.html")]
struct HelloTemplate<'a, T>
where
    T: Display,
{
    name: &'a T,
}

async fn hello() -> impl IntoResponse {
    HelloTemplate { name: &"world" }
}

struct HelloApp;

impl App for HelloApp {
    fn name(&self) -> &'static str {
        "hello-app"
    }

    fn router(&self) -> Router {
        Router::with_urls([Route::with_handler("/", hello)])
    }
}

struct HelloProject;

impl Project for HelloProject {
    fn config(&self, _config_name: &str) -> cot::Result<ProjectConfig> {
        Ok(ProjectConfig::dev_default())
    }

    fn register_apps(&self, apps: &mut AppBuilder, _context: &RegisterAppsContext) {
        apps.register_with_views(HelloApp, "");
    }
}

#[cot::test]
async fn hello_warp_0_3() {
    let mut client = Client::new(HelloProject).await;
    let resp = client.get("/").await.unwrap();
    assert_eq!(resp.status(), 200);
    assert_eq!(
        resp.headers().get("content-type").unwrap(),
        "text/html; charset=utf-8"
    );
    let body = resp.into_body().into_bytes().await.unwrap();
    assert_eq!(&body[..], b"Hello, world!");
}
