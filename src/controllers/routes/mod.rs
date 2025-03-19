use crate::controllers::pages::{
    welcome::welcome, dapp::dapp, index::index, rpc::rpc, security::security,
    nfts::nfts, tokens::tokens, settings::settings, wallets::wallets,
};
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .route("/", web::get().to(index))
            .route("/welcome", web::get().to(welcome))
            .route("/dapp", web::get().to(dapp))
            .route("/rpc", web::get().to(rpc))
            .route("/nfts", web::get().to(nfts))
            .route("/settings", web::get().to(settings))
            .route("/security", web::get().to(security))
            .route("/tokens", web::get().to(tokens))
            .route("/wallets", web::get().to(wallets))
    );
}
