mod config;
mod models;
mod repositorios;
mod traits;

use models::aluno::Aluno;
use models::aluno_nota::AlunoNota;
use models::materia::Materia;
use repositorios::repositorio_orm::RepositorioORM;

fn main(){
    let str_connection = config::configuration::get_mysql_string_connection();

    let repo_aluno = RepositorioORM::<Aluno>::new(&str_connection);
    let repo_aluno_nota = RepositorioORM::<AlunoNota>::new(&str_connection);
    let repo_materia = RepositorioORM::<Materia>::new(&str_connection);

    repo_aluno.criar_tabela();
    repo_aluno_nota.criar_tabela();
    repo_materia.criar_tabela();
    println!("Tabelas criadas com sucesso!");


    let aluno = Aluno{
        id:0,
        nome: "Danilo".to_string(),
        matricula: "001DA".to_string()
    };
    let aluno_id = repo_aluno.incluir(&aluno);
    println!("Aluno ({}) incluido com sucesso!", aluno_id);


    let aluno_nota = AlunoNota{
        id: 0,
        aluno_id: aluno_id,
        nota: 10.0,
    };
    repo_aluno_nota.incluir(&aluno_nota);
    println!("Nota aluno ({}) incluido com sucesso!", aluno_id);



    let materia = Materia{
        id:0,
        titulo: "Matemática".to_string(),
        descricao: "Matemática para logica de programação".to_string()
    };
    let materia_id = repo_materia.incluir(&materia);
    println!("Materia ({}) incluido com sucesso!", materia_id);
}