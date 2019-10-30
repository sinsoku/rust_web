use actix_web::{web, HttpResponse, Responder};
use handlebars::Handlebars;

pub fn index(hb: web::Data<Handlebars>) -> impl Responder {
    let data = json!({
        "caption": "Hello world!"
    });
    let res = hb.render("top/index", &data).unwrap();

    HttpResponse::Ok().body(res)
}
