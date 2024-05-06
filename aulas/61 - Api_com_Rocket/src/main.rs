#[macro_use] extern crate rocket;
extern crate orm_desafio_v1;
use middlewares::auth_fairing::AuthFairing;

mod controllers;
mod dtos;
mod config;
mod models;
mod servicos;
mod model_views;
mod middlewares;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![
            controllers::home_controller::index,

            // controllers::login_controller::logar,

            controllers::alunos_controller::index,
            controllers::alunos_controller::show,
            controllers::alunos_controller::criar,
            controllers::alunos_controller::excluir,
            controllers::alunos_controller::alterar,
        ])
        .attach(AuthFairing)
}
