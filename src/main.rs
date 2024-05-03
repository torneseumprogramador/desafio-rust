#[macro_use] extern crate rocket;
extern crate orm_desafio_v1;
use rocket::fs::{FileServer, relative};
use rocket_dyn_templates::Template;

mod controllers;
mod dtos;
mod config;
mod models;
mod servicos;
mod model_views;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/static", FileServer::from(relative!("static")))
        .mount("/", routes![
            controllers::home_controller::index,
            controllers::home_controller::sobre,

            controllers::alunos_controller::index,
            controllers::alunos_controller::excluir,
            controllers::alunos_controller::novo,
            controllers::alunos_controller::criar,
        ])
        .attach(Template::fairing())
}
