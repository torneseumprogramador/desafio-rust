use crate::model_views::home_view::HomeView;
use axum::Json;

pub async fn index() -> Json<HomeView> {
    Json(HomeView { mensagem: String::from("Bem vindo a API Axum com Rust") })
}