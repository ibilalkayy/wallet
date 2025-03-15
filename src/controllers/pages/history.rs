use actix_web::{HttpResponse, Responder, web};
use tera::{Context, Tera};

pub async fn history(tmpl: web::Data<Tera>) -> impl Responder {
    let ctx = Context::new();
    let s = tmpl.render("history.html", &ctx).unwrap();
    HttpResponse::Ok().body(s)
}
