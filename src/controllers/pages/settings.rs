use actix_web::{HttpResponse, Responder, web};
use tera::{Context, Tera};

use crate::controllers::middleware::middleware::check_login;

pub async fn settings(tmpl: web::Data<Tera>) -> impl Responder {
    if let Some(resource) = check_login() {
        return resource;
    }
    let ctx = Context::new();
    let s = tmpl.render("settings.html", &ctx).unwrap();
    HttpResponse::Ok().body(s)
}
