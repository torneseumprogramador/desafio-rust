extern crate orm_desafio_v1;
mod config;
mod handlers;
mod models;
mod services;

use actix_web::{App, HttpServer};
use crate::config::load_config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cfg = load_config().expect("Failed to load configuration.");

    let server_cfg = cfg.server;
    let address = format!("{}:{}", server_cfg.host, server_cfg.port);

    println!("Iniciando o servidor em http://{}", address);

    HttpServer::new(|| { App::new().configure(config::routes) })
    .bind(&address)?
    .run()
    .await
}