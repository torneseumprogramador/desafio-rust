use config::{Config, ConfigError, File, Environment};
use serde::Deserialize;
use actix_web::web;
use crate::handlers;

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
}
