use actix_web::{HttpResponse, Responder, web};
use tera::{Context, Tera};

pub async fn rpc(tmpl: web::Data<Tera>) -> impl Responder {
    let ctx = Context::new();
    let s = tmpl.render("rpc.html", &ctx).unwrap();
    HttpResponse::Ok().body(s)
}
