use actix_web::{HttpResponse, Responder, web};
use tera::{Context, Tera};

pub async fn wallet(tmpl: web::Data<Tera>) -> impl Responder {
    let ctx = Context::new();
    let s = tmpl.render("wallet.html", &ctx).unwrap();
    HttpResponse::Ok().body(s)
}
