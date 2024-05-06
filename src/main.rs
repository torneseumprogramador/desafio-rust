#[macro_use] extern crate rocket;
extern crate orm_desafio_v1;
use rocket::fs::{FileServer, relative};
use rocket_dyn_templates::Template;
use middlewares::authentication::CookieAuthFairing;

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
        .mount("/static", FileServer::from(relative!("static")))
        .mount("/", routes![
            controllers::home_controller::index,
            controllers::home_controller::sobre,

            controllers::login_controller::index,
            controllers::login_controller::logar,
            controllers::login_controller::sair,

            controllers::alunos_controller::index,
            controllers::alunos_controller::excluir,
            controllers::alunos_controller::novo,
            controllers::alunos_controller::criar,
            controllers::alunos_controller::editar,
            controllers::alunos_controller::alterar,
        ])
        .attach(Template::fairing())
        .attach(CookieAuthFairing) // Anexar o Fairing de autenticação via cookie
}
