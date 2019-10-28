use crate::controllers::{posts, top};
use actix_web::web;

pub fn top(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(top::index));
}

pub fn posts(cfg: &mut web::ServiceConfig) {
    cfg.route("/posts", web::get().to(posts::index))
        .route("/posts/{id}", web::get().to(posts::show))
        .route("/posts", web::post().to(posts::create))
        .route("/posts/{id}", web::put().to(posts::update))
        .route("/posts/{id}", web::patch().to(posts::update))
        .route("/posts/{id}", web::delete().to(posts::destroy));
}
