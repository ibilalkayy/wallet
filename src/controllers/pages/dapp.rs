use actix_web::{HttpResponse, Responder, web};
use tera::{Context, Tera};

pub async fn dapp(tmpl: web::Data<Tera>) -> impl Responder {
    let ctx = Context::new();
    let s = tmpl.render("dapp.html", &ctx).unwrap();
    HttpResponse::Ok().body(s)
}
