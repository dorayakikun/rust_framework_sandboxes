#![deny(warnings)]
extern crate pretty_env_logger;
extern crate warp;

use warp::Filter;

fn main() {
    pretty_env_logger::init();

    let v1 = warp::path("v1").map(|| "v1!");

    let v2 = warp::path("v2").map(|| "v2!");

    let routes = warp::get2().and(v1.or(v2));

    warp::serve(routes).run(([127, 0, 0, 1], 3030));
}
