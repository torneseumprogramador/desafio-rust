use config::{Config, ConfigError, File, Environment};
use serde::Deserialize;
use actix_web::web;
use crate::handlers;
use crate::middleware::AuthMiddleware;
use std::env;

pub fn get_mysql_string_connection() -> String {
    let user:String = match env::var("DATABASE_USER") {
        Ok(value) => value,
        Err(_) => "root".to_string(),
    };

    let pass:String = match env::var("DATABASE_PASSWORD") {
        Ok(value) => value,
        Err(_) => "".to_string(),
    };

    let db:String = match env::var("DATABASE_DB") {
        Ok(value) => value,
        Err(_) => "seu_db".to_string(),
    };

    let host:String = match env::var("DATABASE_HOST") {
        Ok(value) => value,
        Err(_) => "localhost".to_string(),
    };

    format!("mysql://{}:{}@{}/{}", user, pass, host, db)
}

#[derive(Debug, Deserialize)]
pub struct ServerData {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct JwtConfig {
    pub secret: String,
}

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub server: ServerData,
    pub jwt: JwtConfig,
}

pub fn load_config() -> Result<AppConfig, ConfigError> {
    let mut cfg = Config::new();
    cfg.merge(File::with_name("ActixWeb"))?;
    cfg.merge(Environment::with_prefix("APP"))?;
    cfg.try_into()
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(handlers::index));
    cfg.route("/logar", web::post().to(handlers::logar));
    cfg.service(
        web::scope("/alunos")
            .wrap(AuthMiddleware)
            .route("", web::get().to(handlers::alunos_index))
            .route("", web::post().to(handlers::alunos_criar))
            .route("/{id}", web::get().to(handlers::alunos_show))
            .route("/{id}", web::put().to(handlers::alunos_atualizar))
            .route("/{id}", web::delete().to(handlers::alunos_apagar))
    );
}
