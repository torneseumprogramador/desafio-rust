mod model_views;
mod dtos;
mod controllers;

use axum::{
    routing::{get, post, delete, put},
    Router,
};

use crate::controllers::{ home_controller, alunos_controller };

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(home_controller::index))

        .route("/alunos", get(alunos_controller::index))
        .route("/alunos/:id", get(alunos_controller::mostrar))
        .route("/alunos", post(alunos_controller::criar))
        .route("/alunos/:id", delete(alunos_controller::apagar))
        .route("/alunos/:id", put(alunos_controller::alterar));

    println!("::: Sevidor rodando no http://localhost:3001 :::");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}