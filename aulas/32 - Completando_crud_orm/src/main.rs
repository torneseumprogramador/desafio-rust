mod config;
mod models;
mod repositorios;
mod traits;

use models::aluno::Aluno;
use models::aluno_nota::AlunoNota;
use models::materia::Materia;
use repositorios::repositorio_orm::RepositorioORM;
use serde_json::Value;
use std::collections::HashMap;

fn main(){
    let str_connection = config::configuration::get_mysql_string_connection();

    let repo_aluno = RepositorioORM::<Aluno>::new(&str_connection);
    let repo_aluno_nota = RepositorioORM::<AlunoNota>::new(&str_connection);
    let repo_materia = RepositorioORM::<Materia>::new(&str_connection);

    // repo_aluno.criar_tabela();
    // repo_aluno_nota.criar_tabela();
    // repo_materia.criar_tabela();
    // println!("Tabelas criadas com sucesso!");


    // let aluno = Aluno{
    //     id: 0,
    //     nome: "Bruno".to_string(),
    //     matricula: "002BE".to_string()
    // };
    // let aluno_id = repo_aluno.incluir(&aluno);
    // println!("Aluno ({}) incluido com sucesso!", aluno_id);


    // let aluno_modificar = Aluno{
    //     id: 1,
    //     nome: "Danilo Santos".to_string(),
    //     matricula: "001DAN".to_string()
    // };

    // repo_aluno.atualizar(&aluno_modificar);
    // println!("Aluno atualizado com sucesso");


    // repo_aluno.apagar_por_id(1);
    // println!("Aluno apagado com sucesso");


    // let aluno_nota = AlunoNota{
    //     id: 0,
    //     aluno_id: aluno_id,
    //     nota: 10.0,
    // };
    // repo_aluno_nota.incluir(&aluno_nota);
    // println!("Nota aluno ({}) incluido com sucesso!", aluno_id);



    // let materia = Materia{
    //     id:0,
    //     titulo: "Matemática".to_string(),
    //     descricao: "Matemática para logica de programação".to_string()
    // };
    // let materia_id = repo_materia.incluir(&materia);
    // println!("Materia ({}) incluido com sucesso!", materia_id);





    println!("-------[Lista Aluno]------");

    let alunos = repo_aluno.todos();
    for aluno in alunos {
        println!("{:?}", aluno);
        println!("-----------");
    }

    println!("-------[Lista Aluno Nota]------");

    let alunos_notas = repo_aluno_nota.todos();
    for aluno_nota in alunos_notas {
        println!("{:?}", aluno_nota);
        println!("-----------");
    }

    println!("-------[Lista Materia]------");

    let materias = repo_materia.todos();
    for materia in materias {
        println!("{:?}", materia);
        println!("-----------");
    }



    println!("-------[aluno_por_id]------");
    let aluno_por_id = repo_aluno.buscar_por_id(1);
    println!("{:?}", aluno_por_id);



    println!("-------[busca com where de matricula]------");
    let mut params = HashMap::new();
    params.insert("matricula".to_string(), Value::from("001DA"));

    let resultado_matricula = repo_aluno.where_query("matricula = :matricula".to_string(), params);
    println!("{:?}", resultado_matricula);


    println!("-------[busca com where like de nome]------");
    let mut params = HashMap::new();
    params.insert("nome".to_string(), Value::from("%Da%"));

    let resultado_nome = repo_aluno.where_query("nome like :nome".to_string(), params);
    println!("{:?}", resultado_nome);



    println!("-------[busca com limit para paginação]------");
    let mut params = HashMap::new();
    params.insert("nome".to_string(), Value::from("%Da%"));

    let resultado_nome = repo_aluno.where_query("nome like :nome limit 1".to_string(), params);
    println!("{:?}", resultado_nome);
}