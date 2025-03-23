use wallet::controllers;

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};
    use tera::Tera;

    #[actix_web::test]
    async fn test_main() {
        let tera = Tera::new("src/views/*").unwrap();
        let app = test::init_service(
            App::new()
                .app_data(actix_web::web::Data::new(tera))
                .configure(controllers::routes::configure),
        )
        .await;

        let req = test::TestRequest::get().uri("/").to_request();
        let resp = test::call_service(&app, req).await;

        // Extract status before reading the body
        let status = resp.status();

        // Assert the status
        assert!(status.is_success(), "Response was not successful!");
    }
}