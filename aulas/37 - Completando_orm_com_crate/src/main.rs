extern crate orm_desafio_v1;

mod config;
mod models;

use models::aluno::Aluno;
use models::aluno_nota::AlunoNota;
use models::materia::Materia;
use orm_desafio_v1::repositorio_orm_mysql::RepositorioOrmMySql;
use std::collections::HashMap;

fn main(){
    let str_connection = config::configuration::get_mysql_string_connection();

    let repo_aluno = RepositorioOrmMySql::<Aluno>::new(&str_connection);
    repo_aluno.apagar_tabela();
    repo_aluno.criar_tabela();

    let repo_materia = RepositorioOrmMySql::<Materia>::new(&str_connection);
    repo_materia.apagar_tabela();
    repo_materia.criar_tabela();

    let repo_aluno_nota = RepositorioOrmMySql::<AlunoNota>::new(&str_connection);
    repo_aluno_nota.apagar_tabela();
    repo_aluno_nota.criar_tabela();


    let aluno = Aluno{
        id: 0,
        nome: "Bruno".to_string(),
        matricula: "002BE".to_string()
    };
    let aluno_id = repo_aluno.incluir(&aluno);
    println!("Aluno ({}) incluido com sucesso!", aluno_id);

    let aluno_apagar = Aluno{
        id: 0,
        nome: "Aluno Apagar".to_string(),
        matricula: "002APAGAR".to_string()
    };
    let aluno_id_apagar = repo_aluno.incluir(&aluno_apagar);

    let aluno_modificar = Aluno{
        id: 1,
        nome: "Danilo Santos".to_string(),
        matricula: "001DAN".to_string()
    };

    repo_aluno.atualizar(&aluno_modificar);
    println!("Aluno atualizado com sucesso");


    repo_aluno.apagar_por_id(aluno_id_apagar);
    println!("Aluno apagado com sucesso");


    println!("-------[Lista Aluno]------");
    let alunos = repo_aluno.todos();
    for aluno in alunos {
        println!("{:?}", aluno);
        println!("-----------");
    }

    
    println!("-------[aluno_por_id]------");
    let aluno_por_id = repo_aluno.buscar_por_id(aluno_id);
    println!("{:?}", aluno_por_id);


    println!("-------[busca com where de matricula]------");
    let mut params = HashMap::new();
    params.insert("matricula".to_string(), String::from("001DAN"));
    let resultado_matricula = repo_aluno.where_query("matricula = :matricula".to_string(), &params);
    println!("{:?}", resultado_matricula);


    println!("-------[busca com where like de nome]------");
    let mut params = HashMap::new();
    params.insert("nome".to_string(), String::from("%Da%"));
    let resultado_nome = repo_aluno.where_query("nome like :nome".to_string(), &params);
    println!("{:?}", resultado_nome);


    println!("-------[busca com limit para paginação]------");
    let resultado_nome = repo_aluno.where_query("1=1 LIMIT 1 OFFSET 0".to_string(), &HashMap::new());
    println!("{:?}", resultado_nome);



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

}