use actix_web::{HttpResponse, Responder, web};
use tera::{Context, Tera};
use crate::controllers::{
    middleware::middleware::check_login,
    home::home::check_balance,
};

pub async fn index(tmpl: web::Data<Tera>) -> impl Responder {
    let balance = check_balance().await.unwrap();
    if let Some(response) = check_login() {
        return response;
    }
    let mut ctx = Context::new();
    ctx.insert("balance", &balance);
    let s = tmpl.render("index.html", &ctx).unwrap();
    HttpResponse::Ok().body(s)
}
