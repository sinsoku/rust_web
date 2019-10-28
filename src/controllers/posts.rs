use crate::establish_connection;
use crate::models::{NewPost, Post};
use crate::schema;
use actix_web::{HttpRequest, HttpResponse, Responder};
use diesel::prelude::*;

pub fn index() -> impl Responder {
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

pub fn show(req: HttpRequest) -> impl Responder {
    use schema::posts::dsl::posts;

    let id: i32 = req.match_info().get("id").unwrap().parse().unwrap();
    let connection = establish_connection();
    let post = posts
        .find(id)
        .first::<Post>(&connection)
        .expect("Error finding posts");
    let res = format!("Show {}", post.id);

    HttpResponse::Ok().body(res)
}

pub fn create() -> impl Responder {
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

pub fn update(req: HttpRequest) -> impl Responder {
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

pub fn destroy(req: HttpRequest) -> impl Responder {
    use schema::posts::dsl::posts;

    let id: i32 = req.match_info().get("id").unwrap().parse().unwrap();
    let connection = establish_connection();
    diesel::delete(posts.find(id))
        .execute(&connection)
        .expect("Error deleting posts");

    HttpResponse::NoContent()
}
