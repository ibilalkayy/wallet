use actix_web::{HttpResponse, Responder, web};
use tera::{Context, Tera};
use crate::controllers::middleware::middleware::check_login;

pub async fn send(tmpl: web::Data<Tera>) -> impl Responder {
    if let Some(response) = check_login() {
        return response;
    }
    let ctx = Context::new();
    let s = tmpl.render("send.html", &ctx).unwrap();
    HttpResponse::Ok().body(s)
}
