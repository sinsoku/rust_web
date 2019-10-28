extern crate rust_web;

use actix_web::{App, HttpServer};
use rust_web::routes;

fn main() {
    HttpServer::new(|| App::new().configure(routes::top).configure(routes::posts))
        .bind("127.0.0.1:8088")
        .unwrap()
        .run()
        .unwrap();
}
