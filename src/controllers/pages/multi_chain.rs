use actix_web::{HttpResponse, Responder, web};
use tera::{Context, Tera};

pub async fn multi_chain(tmpl: web::Data<Tera>) -> impl Responder {
    let ctx = Context::new();
    let s = tmpl.render("multi_chain.html", &ctx).unwrap();
    HttpResponse::Ok().body(s)
}
