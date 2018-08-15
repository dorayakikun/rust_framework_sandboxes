extern crate actix_web;
use actix_web::{http::Method, server, App, HttpResponse};

fn main() {
    server::new(|| {
        vec![
            App::new()
                .prefix("/v1")
                .resource("/", |r| r.method(Method::GET).f(|_r| HttpResponse::Ok())),
            App::new()
                .prefix("/v2")
                .resource("/", |r| r.method(Method::GET).f(|_r| HttpResponse::Ok())),
        ]
    }).bind("localhost:3030")
    .unwrap()
    .run();
}
