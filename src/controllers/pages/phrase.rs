use actix_web::{HttpResponse, Responder, web};
use tera::{Context, Tera};

pub async fn phrase(tmpl: web::Data<Tera>) -> impl Responder {
    let ctx = Context::new();
    let s = tmpl.render("phrase.html", &ctx).unwrap();
    HttpResponse::Ok().body(s)
}

pub async fn phrase_continue(tmpl: web::Data<Tera>) -> impl Responder {
    let mut ctx = Context::new();
    ctx.insert("continued", &true);
    let s = tmpl.render("message.html", &ctx).unwrap();
    HttpResponse::Ok().body(s)
}