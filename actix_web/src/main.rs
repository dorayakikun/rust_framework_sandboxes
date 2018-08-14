extern crate actix_web;
use actix_web::{http, server, App, Path, Responder};

fn index(info: Path<()>) -> impl Responder {
    format!("Hello, World!")
}

fn main() {
    server::new(|| App::new()
        .route("/", http::Method::GET, index))
        .bind("localhost:3030").unwrap()
        .run();
}
