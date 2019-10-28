use actix_web::{HttpResponse, Responder};

pub fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
