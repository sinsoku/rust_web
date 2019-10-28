use crate::models::Post;
use actix_web::{HttpRequest, HttpResponse, Responder};

pub fn index() -> impl Responder {
    let results = Post::all();
    let res = format!("Displaying {} posts", results.len());

    HttpResponse::Ok().body(res)
}

pub fn show(req: HttpRequest) -> impl Responder {
    let post = find_post(req);
    let res = format!("Show {}", post.id);

    HttpResponse::Ok().body(res)
}

pub fn create() -> impl Responder {
    Post::create("title", "body");

    HttpResponse::Created().body("Inserting")
}

pub fn update(req: HttpRequest) -> impl Responder {
    let mut post = find_post(req);
    post.publish();

    HttpResponse::Ok().body("Published")
}

pub fn destroy(req: HttpRequest) -> impl Responder {
    let post = find_post(req);
    post.destroy();

    HttpResponse::NoContent()
}

fn find_post(req: HttpRequest) -> Post {
    let id: i32 = req.match_info().get("id").unwrap().parse().unwrap();
    Post::find(id)
}
