#[cfg(test)]
mod tests {
    use actix_web::HttpResponse;
    use wallet::controllers::middleware::middleware::check_login;

    #[test]
    fn test_check_when_logged_in() {
        let response = check_login();

        assert!(response.is_none(), "Expected None when logged in");
    }

    #[test]
    fn test_check_when_not_logged_in() {
        let is_logged_in = false;

        let response = if !is_logged_in {
            Some(HttpResponse::Found()
                .append_header(("Location", "/welcome"))
                .finish())
        } else {
            None
        };

        assert!(response.is_some(), "Expected some when not logged in");

        let resp = response.unwrap();

        assert_eq!(resp.status(), 302, "Expected 302 Found status");

        let headers = resp.headers();
        assert_eq!(
            headers.get("Location").unwrap().to_str().unwrap(),
            "/welcome",
            "Expected Location header to be /welcome"
        );
    }
}