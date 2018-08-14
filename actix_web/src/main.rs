extern crate actix_web;
use actix_web::{server, App, HttpRequest};

fn index(_req: &HttpRequest) -> &'static str {
    "Hello, World!"
}

fn main() {
    server::new(|| App::new().resource("/", |r| r.f(index)))
        .bind("localhost:3030")
        .unwrap()
        .run();
}
