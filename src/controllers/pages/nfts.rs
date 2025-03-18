use actix_web::{HttpResponse, Responder, web};
use tera::{Context, Tera};

pub async fn nfts(tmpl: web::Data<Tera>) -> impl Responder {
    let ctx = Context::new();
    let s = tmpl.render("nfts.html", &ctx).unwrap();
    HttpResponse::Ok().body(s)
}
