use crate::controllers::pages::{
    welcome::welcome, dapp::dapp, index::index, rpc::rpc, security::security, phrase::{phrase, phrase_continue},
    nfts::nfts, tokens::tokens, settings::settings, wallets::wallets, wallet::wallet, send::send,
};
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .route("/", web::get().to(index))
            .route("/welcome", web::get().to(welcome))
            .route("/wallet", web::get().to(wallet))
            .route("/phrase", web::post().to(phrase))
            .route("/phrase/continue", web::post().to(phrase_continue))
            .route("/dapp", web::get().to(dapp))
            .route("/rpc", web::get().to(rpc))
            .route("/nfts", web::get().to(nfts))
            .route("/settings", web::get().to(settings))
            .route("/security", web::get().to(security))
            .route("/tokens", web::get().to(tokens))
            .route("/wallets", web::get().to(wallets))
            .route("/send", web::get().to(send))
    );
}
