mod model_views;
mod controllers;
mod dtos;

use tide::Result;
use crate::controllers::home_controller;
use crate::controllers::alunos_controller;

#[async_std::main]
async fn main() -> Result<()> {
    let mut app = tide::new();

    app.at("/").get(home_controller::index);
    app.at("/alunos").post(alunos_controller::criar);
    app.at("/alunos/:id").get(alunos_controller::mostrar);

    app.listen("127.0.0.1:8080").await?;

    Ok(())
}
