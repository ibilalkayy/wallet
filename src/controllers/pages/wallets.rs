use actix_web::{HttpResponse, Responder, web};
use tera::{Context, Tera};

pub async fn wallets(tmpl: web::Data<Tera>) -> impl Responder {
    let ctx = Context::new();
    let s = tmpl.render("wallets.html", &ctx).unwrap();
    HttpResponse::Ok().body(s)
}
