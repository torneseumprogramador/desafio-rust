#[macro_use] extern crate rocket;
extern crate orm_desafio_v1;
use rocket::fs::{FileServer, relative};
use rocket_dyn_templates::Template;

mod controllers;
mod config;
mod models;
mod servicos;

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
        ])
        .attach(Template::fairing())
}
