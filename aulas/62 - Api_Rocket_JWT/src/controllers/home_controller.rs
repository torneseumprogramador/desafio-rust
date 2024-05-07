use rocket::serde::json::Json;
use crate::model_views::home_view::HomeView;

#[get("/")]
pub fn index() -> Json<HomeView> {
    Json( HomeView { mensagem: String::from("Bem vindo a API com Rocket em Rust") } )
}