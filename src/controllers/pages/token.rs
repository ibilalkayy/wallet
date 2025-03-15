use actix_web::{HttpResponse, Responder, web};
use tera::{Context, Tera};

pub async fn token(tmpl: web::Data<Tera>) -> impl Responder {
    let ctx = Context::new();
    let s = tmpl.render("token.html", &ctx).unwrap();
    HttpResponse::Ok().body(s)
}
