mod config;
mod models;
mod repositorios;
mod traits;

use models::aluno::Aluno;
use repositorios::repositorio_orm::RespositorioORM;

fn main(){
    let aluno = Aluno{
        id:0,
        nome: "Danilo".to_string(),
        matricula: "001DA".to_string()
    };

    let str_connection = config::configuration::get_mysql_string_connection();
    let repo = RespositorioORM::new(&str_connection);

    repo.criar_tabela(aluno);

    println!("Tabela criada com sucesso!");
}