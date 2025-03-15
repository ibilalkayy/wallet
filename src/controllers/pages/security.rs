use actix_web::{HttpResponse, Responder, web};
use tera::{Context, Tera};

pub async fn security(tmpl: web::Data<Tera>) -> impl Responder {
    let ctx = Context::new();
    let s = tmpl.render("security.html", &ctx).unwrap();
    HttpResponse::Ok().body(s)
}
