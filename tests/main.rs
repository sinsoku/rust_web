#[cfg(test)]
mod tests {
    extern crate rust_web;

    use actix_web::dev::Service;
    use actix_web::http::StatusCode;
    use actix_web::{test, web, App};
    use bytes::Bytes;
    use handlebars::Handlebars;
    use rust_web::routes;
    use std::str;

    #[test]
    fn get_root() {
        let mut handlebars = Handlebars::new();
        handlebars
            .register_templates_directory(".hbs", "./src/templates")
            .unwrap();
        let handlebars_ref = web::Data::new(handlebars);

        let mut app = test::init_service(
            App::new()
                .register_data(handlebars_ref.clone())
                .configure(routes::top)
                .configure(routes::posts),
        );
        let req = test::TestRequest::get().uri("/").to_request();
        let resp = test::block_on(app.call(req)).unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
        let html = str::from_utf8(&test::read_body(resp))
            .expect("Could not convert response to UTF8")
            .to_string();
        assert!(html.contains("<h1>Hello world!</h1>"));
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
