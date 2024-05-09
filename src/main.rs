#[macro_use] extern crate rocket;
extern crate orm_desafio_v1;
use middlewares::auth_fairing::AuthFairing;
use rocket::http::Status;
use middlewares::cors::Cors;

mod controllers;
mod dtos;
mod config;
mod models;
mod servicos;
mod model_views;
mod middlewares;

/*
use crate::servicos::usuario_servico;
use crate::models::usuario::Usuario;
use crate::orm_desafio_v1::repositorio_orm_mysql::RepositorioOrmMySql;
*/

#[options("/<_..>")]
fn all_options() -> Status {
    Status::Ok
}

#[launch]
fn rocket() -> _ {
    /*
    
    let sql_connection = config::configuration::get_mysql_string_connection();
    RepositorioOrmMySql::<Usuario>::new(&sql_connection).criar_tabela();

    usuario_servico::incluir(Usuario {
        id: 0,
        nome: String::from("Danilo"),
        email: String::from("danilo@teste.com"),
        senha: String::from("123456"),
    });

    */

    rocket::build()
        .mount("/", routes![
            all_options,

            controllers::home_controller::index,

            controllers::login_controller::logar,
            controllers::login_controller::unauthorized,

            controllers::alunos_controller::index,
            controllers::alunos_controller::show,
            controllers::alunos_controller::criar,
            controllers::alunos_controller::excluir,
            controllers::alunos_controller::alterar,
        ])
        .attach(AuthFairing)
        .attach(Cors)
}
