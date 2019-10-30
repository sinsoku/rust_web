#[cfg(test)]
mod tests {
    extern crate rust_web;

    use actix_web::dev::Service;
    use actix_web::http::StatusCode;
    use actix_web::{test, App};
    use rust_web::models::Post;
    use rust_web::routes;

    #[test]
    fn get_posts_json() {
        Post::create("a", "b");
        let posts = Post::all();
        let post = posts.last().unwrap();
        let uri = format!("/posts/{}.json", post.id);

        let mut app =
            test::init_service(App::new().configure(routes::top).configure(routes::posts));
        let req = test::TestRequest::get().uri(&uri).to_request();
        let resp = test::block_on(app.call(req)).unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
        let json_u8 = test::read_body(resp).to_vec();
        let json = unsafe { String::from_utf8_unchecked(json_u8) };
        let json_post: Post = serde_json::from_str(&json).unwrap();

        assert_eq!(json_post.id, post.id);
        assert_eq!(json_post.title, "a");
        assert_eq!(json_post.body, "b");

        post.destroy();
    }
}
