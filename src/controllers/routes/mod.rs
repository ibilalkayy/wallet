use crate::controllers::pages::{
    account::account, dapp::dapp, history::history, index::index, manage::manage,
    multi_chain::multi_chain, rpc::rpc, security::security, token::token, transfer::transfer,
};
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .route("/", web::get().to(index))
            .route("/account", web::get().to(account))
            .route("/dapp", web::get().to(dapp))
            .route("/history", web::get().to(history))
            .route("/manage", web::get().to(manage))
            .route("/multi_chain", web::get().to(multi_chain))
            .route("/rpc", web::get().to(rpc))
            .route("/security", web::get().to(security))
            .route("/token", web::get().to(token))
            .route("/transfer", web::get().to(transfer)),
    );
}
