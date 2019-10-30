extern crate rust_web;

use actix_web::{web, App, HttpServer};
use handlebars::Handlebars;
use rust_web::routes;

fn main() {
    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".hbs", "./src/templates")
        .unwrap();
    let handlebars_ref = web::Data::new(handlebars);

    HttpServer::new(move || {
        App::new()
            .register_data(handlebars_ref.clone())
            .configure(routes::top)
            .configure(routes::posts)
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap();
}
