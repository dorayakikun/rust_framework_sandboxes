extern crate actix;
extern crate actix_web;
use actix_web::{server::HttpServer, App, HttpResponse};

fn main() {
    let sys = actix::System::new("guide");

    HttpServer::new(|| App::new().resource("/", |r| r.f(|_| HttpResponse::Ok())))
        .bind("127.0.0.1:59080")
        .unwrap()
        .start();

    // run()を利用するとactix_webを別スレッドで実行するしてくれる
    let _ = sys.run();
}
