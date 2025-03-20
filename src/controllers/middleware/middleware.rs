use actix_web::HttpResponse;

pub fn check_login() -> Option<HttpResponse> {
    let is_logged_in = false;
    if !is_logged_in {
        Some(HttpResponse::Found().append_header(("Location", "/welcome")).finish())
    } else {
        None
    }
}