extern crate orm_desafio_v1;
mod config;
mod handlers;
mod models;
mod services;
mod jwt;
mod middleware;

use actix_web::{App, HttpServer};
use crate::config::load_config;
use actix_cors::Cors;
use actix_web::http;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cfg = load_config().expect("Failed to load configuration.");

    let server_cfg = cfg.server;
    let address = format!("{}:{}", server_cfg.host, server_cfg.port);

    println!("Iniciando o servidor em http://{}", address);
    
    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin_fn(|origin, _req_head| true)  // Permitir todas as origens
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])  // Métodos permitidos
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .supports_credentials()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .configure(config::routes)
    })
    .bind(&address)?
    .run()
    .await
}