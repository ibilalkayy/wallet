use actix_web::{HttpResponse, Responder, web};
use tera::{Context, Tera};

use crate::controllers::middleware::middleware::check_login;

pub async fn rpc(tmpl: web::Data<Tera>) -> impl Responder {
    if let Some(resource) = check_login() {
        return resource;
    }
    let ctx = Context::new();
    let s = tmpl.render("rpc.html", &ctx).unwrap();
    HttpResponse::Ok().body(s)
}
