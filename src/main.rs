extern crate diesel;
extern crate rust_web;

use rust_web::*;
use self::models::{Post, NewPost};
use diesel::prelude::*;

use actix_web::{web, App, HttpResponse, HttpServer, Responder, HttpRequest};

fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

fn posts_index() -> impl Responder {
    use schema::posts::dsl::*;

    let connection = establish_connection();
    let results = posts
        .filter(published.eq(true))
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");
    let res = format!("Displaying {} posts", results.len());

    HttpResponse::Ok().body(res)
}

fn posts_show(req: HttpRequest) -> impl Responder {
    use schema::posts::dsl::posts;

    let id: i32 = req.match_info().get("id").unwrap().parse().unwrap();
    let connection = establish_connection();
    let post = posts.find(id)
        .first::<Post>(&connection)
        .expect("Error finding posts");
    let res = format!("Show {}", post.id);

    HttpResponse::Ok().body(res)
}

fn posts_create() -> impl Responder {
    use schema::posts;

    let new_post = NewPost {
        title: "title",
        body: "body",
    };

    let connection = establish_connection();
    let rows_inserted = diesel::insert_into(posts::table)
        .values(&new_post)
        .execute(&connection)
        .expect("Error saving new post");
    let res = format!("Inserting {} posts", rows_inserted);

    HttpResponse::Created().body(res)
}

fn posts_update(req: HttpRequest) -> impl Responder {
    use schema::posts::dsl::{posts, published};

    let id: i32 = req.match_info().get("id").unwrap().parse().unwrap();
    let connection = establish_connection();

    let num_updated = diesel::update(posts.find(id))
        .set(published.eq(true))
        .execute(&connection)
        .expect(&format!("Unable to find post {}", id));
    let res = format!("Published post {}", num_updated);

    HttpResponse::Ok().body(res)
}

fn posts_destroy(req: HttpRequest) -> impl Responder {
    use schema::posts::dsl::posts;

    let id: i32 = req.match_info().get("id").unwrap().parse().unwrap();
    let connection = establish_connection();
    diesel::delete(posts.find(id))
        .execute(&connection)
        .expect("Error deleting posts");

    HttpResponse::NoContent()
}

fn main() {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/posts", web::get().to(posts_index))
            .route("/posts/{id}", web::get().to(posts_show))
            .route("/posts", web::post().to(posts_create))
            .route("/posts/{id}", web::put().to(posts_update))
            .route("/posts/{id}", web::patch().to(posts_update))
            .route("/posts/{id}", web::delete().to(posts_destroy))
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap();
}
