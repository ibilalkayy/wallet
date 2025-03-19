use actix_web::{HttpResponse, Responder, web};
use tera::{Context, Tera};
use crate::controllers::middleware::middleware::check_login;

pub async fn index(tmpl: web::Data<Tera>) -> impl Responder {
    if let Some(response) = check_login() {
        return response;
    }
    let ctx = Context::new();
    let s = tmpl.render("index.html", &ctx).unwrap();
    HttpResponse::Ok().body(s)
}
