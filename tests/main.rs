#[cfg(test)]
mod tests {
    extern crate rust_web;

    use actix_web::dev::Service;
    use actix_web::http::StatusCode;
    use actix_web::{test, App};
    use bytes::Bytes;
    use rust_web::routes;

    #[test]
    fn get_root() {
        let mut app =
            test::init_service(App::new().configure(routes::top).configure(routes::posts));
        let req = test::TestRequest::get().uri("/").to_request();
        let resp = test::block_on(app.call(req)).unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(test::read_body(resp), Bytes::from_static(b"Hello world!"));
    }

    #[test]
    fn post_posts() {
        let mut app =
            test::init_service(App::new().configure(routes::top).configure(routes::posts));
        let req = test::TestRequest::post().uri("/posts").to_request();
        let resp = test::block_on(app.call(req)).unwrap();

        assert_eq!(resp.status(), StatusCode::CREATED);
        assert_eq!(test::read_body(resp), Bytes::from_static(b"Inserting"));
    }
}
