extern crate orm_desafio_v1;

mod models;
mod ux;
mod negocio;
mod config;

use orm_desafio_v1::repositorio_orm_mysql::RepositorioOrmMySql;
use config::configuration;
use models::aluno::Aluno;
use models::aluno_nota::AlunoNota;
use ux::menu;

fn main() {
    RepositorioOrmMySql::<Aluno>::new(&configuration::get_mysql_string_connection()).criar_tabela();
    RepositorioOrmMySql::<AlunoNota>::new(&configuration::get_mysql_string_connection()).criar_tabela();
    menu::carregar();
}