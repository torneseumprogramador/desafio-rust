use config::{Config, ConfigError, File, Environment};
use serde::Deserialize;
use actix_web::web;
use crate::handlers;
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
pub struct AppConfig {
    pub server: ServerData,
}

pub fn load_config() -> Result<AppConfig, ConfigError> {
    let mut cfg = Config::new();
    cfg.merge(File::with_name("ActixWeb"))?;
    cfg.merge(Environment::with_prefix("APP"))?;
    cfg.try_into()
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(handlers::index));
    cfg.route("/alunos", web::get().to(handlers::alunos_index));
    cfg.route("/alunos/{id}", web::get().to(handlers::alunos_show));
    cfg.route("/alunos/{id}", web::put().to(handlers::alunos_atualizar));
    cfg.route("/alunos/{id}", web::delete().to(handlers::alunos_apagar));
    cfg.route("/alunos", web::post().to(handlers::alunos_criar));
}
