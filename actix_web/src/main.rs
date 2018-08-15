extern crate actix_web;
use actix_web::{http::Method, server, App, HttpRequest, Responder};

fn index(_req: &HttpRequest) -> impl Responder {
    "Hello, Actix Web!"
}

fn main() {
    server::new(|| {
        App::new()
            .prefix("/app")
            .resource("/index.html", |r| r.method(Method::GET).f(index))
    }).bind("localhost:3030")
    .unwrap()
    .run();
}
