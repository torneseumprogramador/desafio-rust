mod model_views;
mod controllers;
mod dtos;

use warp::{Filter};
use crate::controllers::{home_controller, alunos_controller};

#[tokio::main]
async fn main() {
    // Rota raiz
    let root = warp::path::end().map(home_controller::index);

    // Rotas de alunos agrupadas
    let alunos_routes = warp::path("alunos").and(
        warp::get()
            .and(warp::path::end())
            .map(alunos_controller::index)
            .or(
                warp::post()
                .and(warp::path::end())
                .and(warp::body::json())
                .and_then(alunos_controller::criar)
            )
            .or(
                warp::path::param::<i32>()
                .and(warp::get())
                .and_then(alunos_controller::mostrar)
            )
    );

    // Combina todas as rotas
    let routes = root.or(alunos_routes);

    println!("::: App rodando em http://localhost:3030 :::");

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
