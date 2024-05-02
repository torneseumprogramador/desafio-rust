// use std::io;
use crate::models::aluno::Aluno;
use crate::models::aluno_nota::AlunoNota;
use crate::orm_desafio_v1::repositorio_orm_mysql::RepositorioOrmMySql;
use crate::config::configuration;
use std::collections::HashMap;

// pub fn cadastrar_aluno(){
//     if let Some(aluno) = buscar_aluno_por_matricula(&matricula) {
//         let msg = format!("Aluno da matricula {}, já foi cadastrado, seu nome é {}.", aluno.matricula, aluno.nome);
//         return;
//     }

//     let aluno_id = repo().incluir(&Aluno{
//         id: 0,
//         nome: nome.clone(),
//         matricula: matricula
//     });

//     let mut params = HashMap::new();
//     params.insert("aluno_id".to_string(), aluno_id.to_string());
//     repo_nota().apagar_where("aluno_id = :aluno_id".to_string(), &params);
// }

// pub fn alterar_aluno(){

//     if let Some(aluno) = buscar_aluno_por_matricula(&matricula) {

//         repo().atualizar(&Aluno{
//             id: aluno.id,
//             matricula: aluno.matricula,
//             nome: nome.clone()
//         });
//     }

// }


// pub fn excluir_aluno(){
//     if let Some(aluno) = buscar_aluno_por_matricula(&matricula) {
//         let msg = format!("Aluno {} excluido com sucesso.", aluno.nome);
//         repo().apagar_por_id(aluno.id);
//     }
    
// }

// pub fn buscar_aluno_por_matricula(matricula: &str) -> Option<Aluno> {
//     let mut params = HashMap::new();
//     params.insert("matricula".to_string(), matricula.to_string());
//     let alunos = repo().where_query("matricula = :matricula".to_string(), &params);
    
//     if alunos.len() > 0 {
//         return Some(alunos[0].clone());
//     }

//     None
// }


// fn aluno_notas(aluno: &Aluno) -> Vec<f32> {
//     let mut params = HashMap::new();
//     params.insert("aluno_id".to_string(), aluno.id.to_string());
//     let aluno_notas = repo_nota().where_query("aluno_id = :aluno_id".to_string(), &params);
//     aluno_notas.iter().map(|an| an.nota).collect()
// }

// fn aluno_media(aluno: &Aluno) -> f32 {
//     let notas = aluno_notas(aluno);

//     let soma: f32 = notas.iter().sum();
//     soma / notas.len() as f32
// }

// fn aluno_situacao(aluno: &Aluno) -> String {
//     let media = aluno_media(aluno);

//     if media >= 7.0 {
//         return String::from("Aprovado")
//     }
//     else if media < 5.0 {
//         return String::from("Reprovado")
//     }

//     String::from("Recuperação")
// }



pub fn excluir(id: i32){
    repo_nota().apagar_where("aluno_id = :aluno_id".to_string(), &HashMap::from([
        ("aluno_id".to_string(), id.to_string()),
    ]));

    repo_aluno().apagar_por_id(id);
}

pub fn todos() -> (Vec<Aluno>, Vec<Vec<AlunoNota>>){
    let alunos = repo_aluno().todos();

    let mut alunos_notas:Vec<Vec<AlunoNota>> = Vec::new();
    for aluno in &alunos {
        let notas = repo_nota().where_query("aluno_id = :aluno_id".to_string(), &HashMap::from([
            ("aluno_id".to_string(), aluno.id.to_string()),
        ]));
        
        alunos_notas.push(notas);
    }

    (alunos, alunos_notas)
}

fn repo_aluno() -> RepositorioOrmMySql::<Aluno> {
    let sql_connection = configuration::get_mysql_string_connection();
    return RepositorioOrmMySql::<Aluno>::new(&sql_connection);
}

fn repo_nota() -> RepositorioOrmMySql::<AlunoNota> {
    let sql_connection = configuration::get_mysql_string_connection();
    return RepositorioOrmMySql::<AlunoNota>::new(&sql_connection);
}
