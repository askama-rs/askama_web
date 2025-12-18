# askama_web

[![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/rinja-rs/askama_web/ci.yml?branch=main&style=flat-square&logo=github&logoColor=white "GitHub Workflow Status")](https://github.com/rinja-rs/askama_web/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/askama_web?logo=rust&style=flat-square "Crates.io")](https://crates.io/crates/askama_web)
[![docs.rs](https://img.shields.io/docsrs/askama_web?logo=docsdotrs&style=flat-square&logoColor=white "docs.rs")](https://docs.rs/askama_web/)

A compatibility add-on for [askama](https://lib.rs/crates/askama) to support
many different web frameworks.

## Example

E.g. if you are using [axum](https://lib.rs/crates/axum), then add `askama_web` with
the feature `"axum-0.8"` to your `Cargo.toml`:

```toml
[dependencies]
askama_web = { version = "0.14.0", features = ["axum-0.8"] }
```

Then just add `#[derive(WebTemplate)]` to your Askama templated `struct` or `enum`:

```rust
use askama::Template;
use askama_web::WebTemplate;
use axum::Router;
use axum::routing::get;

#[derive(Template, WebTemplate)]
#[template(path = "hello.html")]
struct HelloTemplate {
    name: String,
}

async fn hello() -> HelloTemplate {
    HelloTemplate {
        name: "world".to_string(),
    }
}

let app = Router::new().route("/", get(hello));
```

By selecting the feature `"axum-0.8"`, `HelloTemplate` will implement [`axum::response::IntoResponse`].
The user will receive a "Status: 200", "Content-Type: text/html; charset=utf-8"
response with the rendered struct as body.

[`axum::response::IntoResponse`]: https://docs.rs/axum/0.8.1/axum/response/trait.IntoResponse.html

## Feature flags / web framework selection

These web frameworks are currently implemented
and can be selected with their respective feature flag:

* `"actix-web-4"`: implements [`Responder`](https://docs.rs/actix-web/4.9.0/actix_web/trait.Responder.html)
  for [actix-web](https://docs.rs/actix-web/4.x.x/) in version 4
* `"axum-0.8"` / `"axum-core-0.5"`: implements [`IntoResponse`](https://docs.rs/axum-core/0.5.0/axum_core/response/trait.IntoResponse.html)
  for [axum](https://docs.rs/axum/0.8.x/) in version 0.8 /
  [axum-core](https://docs.rs/axum-core/0.5.x/) in version 0.5
* `"cot-0.3"`: implements [`IntoResponse`](https://docs.rs/cot/0.3.0/cot/response/trait.IntoResponse.html) for
  [cot](https://docs.rs/cot/0.3.x/) in version 0.3
* `"poem-3"`: implements [`IntoResponse`](https://docs.rs/poem/3.1.7/poem/web/trait.IntoResponse.html) for
  [poem](https://docs.rs/poem/3.x.x/) in version 3.x
* `"rocket-0.5"`: implements [`Responder`](https://docs.rs/rocket/0.5.1/rocket/response/trait.Responder.html) for
  [rocket](https://docs.rs/rocket/0.5.x/) in version 0.5
* `"trillium-0.2"`: implements [`Handler`](https://docs.rs/trillium/0.2.20/trillium/trait.Handler.html) for
  [trillium](https://docs.rs/trillium/0.2.x/) in version 0.2
* `"warp-0.4"`: implements [`Reply`](https://docs.rs/warp/0.4.x/warp/reply/trait.Reply.html) for
  [warp](https://docs.rs/warp/0.4.x/) in version 0.4

As well as these logging / debugging facilities to print error messages
if a template could not be rendered:

* `"eprintln"`: using rust's built-in `eprintln!()` macro
* `"log-0.4"`: using [log](https://docs.rs/log/0.4.x/) as logging framework
* `"tracing-0.1"`: using [tracing](https://docs.rs/tracing/0.1.x/) as logging framework

Some older versions are implemented, too:

* `"axum-0.7"` / `"axum-core-0.4"`: implements [`IntoResponse`](https://docs.rs/axum-core/0.4.x/axum_core/response/trait.IntoResponse.html)
  for [axum](https://docs.rs/axum/0.7.x/) in version 0.7 /
  [axum-core](https://docs.rs/axum-core/0.4.x/) in version 0.4
* `"warp-0.3"`: implements [`Reply`](https://docs.rs/warp/0.3.7/warp/reply/trait.Reply.html) for
  [warp](https://docs.rs/warp/0.3.x/) in version 0.3

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
