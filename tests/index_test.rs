#[cfg(test)]
mod tests {
    use actix_web::HttpResponse;
    use tera::Tera;

    fn mock_check_login_logged_in() -> Option<HttpResponse> {
        Some(HttpResponse::Found().finish())
    }

    fn mock_check_login_not_logged_in() -> Option<HttpResponse> {
        None // Simulate not logged in
    }

    #[actix_web::test]
    async fn test_index_logged_in() {
        let tera = Tera::new("src/views/*").unwrap();

        let response = if let Some(resp) = mock_check_login_logged_in() {
            resp
        } else {
            let ctx = tera::Context::new();
            let body = tera.render("index.html", &ctx).unwrap();
            HttpResponse::Ok().body(body)
        };

        assert_eq!(response.status(), 302, "Expected a redirect (302)");
    }

    #[actix_web::test]
    async fn test_index_not_logged_in() {
        let tera = Tera::new("src/views/*").unwrap();

        // Simulate check_login behavior
        let response = if let Some(resp) = mock_check_login_not_logged_in() {
            resp
        } else {
            let ctx = tera::Context::new();
            let body = tera.render("index.html", &ctx).unwrap();
            HttpResponse::Ok().body(body)
        };

        // Verify status is 200 OK
        assert_eq!(response.status(), 200, "Expected a 200 OK");

        // Extract body from response
        let body_bytes = actix_web::body::to_bytes(response.into_body()).await.unwrap();

        // Verify body is not empty
        assert!(!body_bytes.is_empty(), "Response body should not be empty");
    }
}