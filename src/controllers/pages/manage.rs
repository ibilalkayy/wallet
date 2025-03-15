use actix_web::{HttpResponse, Responder, web};
use tera::{Context, Tera};

pub async fn manage(tmpl: web::Data<Tera>) -> impl Responder {
    let ctx = Context::new();
    let s = tmpl.render("manage.html", &ctx).unwrap();
    HttpResponse::Ok().body(s)
}
