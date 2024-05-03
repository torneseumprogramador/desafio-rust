// use std::io;
use crate::models::aluno::Aluno;
use crate::models::aluno_nota::AlunoNota;
use crate::orm_desafio_v1::repositorio_orm_mysql::RepositorioOrmMySql;
use crate::config::configuration;
use std::collections::HashMap;
use crate::dtos::aluno_dto::AlunoDto;

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

pub fn incluir(aluno_dto: AlunoDto) -> Result<(), String>{
    if aluno_dto.nome.is_empty() {
        return Err(String::from("O nome é obrigatório"));
    }

    if aluno_dto.matricula.is_empty() {
        return Err(String::from("A matricula é obrigatória"));
    }

    if aluno_dto.notas.is_empty() {
        return Err(String::from("Digite as notas separadas por vírgula"));
    }

    let notas_array = string_para_array_notas(&aluno_dto.notas)?;

    let aluno_id = repo_aluno().incluir(&Aluno{
        id: 0,
        nome: aluno_dto.nome,
        matricula: aluno_dto.matricula
    });

    let repositorio_nota = repo_nota();
    for nota in notas_array {
        repositorio_nota.incluir(&AlunoNota {
            id: 0,
            aluno_id: aluno_id,
            nota: nota
        });
    }

    Ok(())
}

fn string_para_array_notas(notas_string: &str) -> Result<Vec<f32>, String> {
    notas_string.split(',')
        .map(|s_nota| s_nota.trim().parse::<f32>().map_err(|_| "Erro ao parsear nota".to_string()))
        .collect()
}

pub fn excluir(id: i32){
    repo_nota().apagar_where("aluno_id = :aluno_id".to_string(), &HashMap::from([
        ("aluno_id".to_string(), id.to_string()),
    ]));

    repo_aluno().apagar_por_id(id);
}

pub fn todos() -> (Vec<Aluno>, Vec<Vec<AlunoNota>>, Vec<f32>, Vec<String>) {
    // TODO aqui está com query N+1 precisa alterar para inner join
    let alunos = repo_aluno().todos();

    let mut alunos_notas: Vec<Vec<AlunoNota>> = Vec::new();
    let mut alunos_media: Vec<f32> = Vec::new();
    let mut alunos_situacao: Vec<String> = Vec::new();

    for aluno in &alunos {
        let notas = repo_nota().where_query("aluno_id = :aluno_id".to_string(), &HashMap::from([
            ("aluno_id".to_string(), aluno.id.to_string()),
        ]));
        
        let notas_array: Vec<f32> = notas.iter().map(|n| n.nota).collect();
        
        // Calculando a média
        let media = if !notas_array.is_empty() {
            notas_array.iter().sum::<f32>() / notas_array.len() as f32
        } else {
            0.0
        };
        alunos_media.push(media);
        
        // Determinando a situação
        let situacao = if media < 5.0 {
            "Reprovação".to_string()
        } else if media < 7.0 {
            "Recuperação".to_string()
        } else {
            "Aprovado".to_string()
        };
        alunos_situacao.push(situacao);

        alunos_notas.push(notas);
    }

    (alunos, alunos_notas, alunos_media, alunos_situacao)
}

fn repo_aluno() -> RepositorioOrmMySql::<Aluno> {
    let sql_connection = configuration::get_mysql_string_connection();
    return RepositorioOrmMySql::<Aluno>::new(&sql_connection);
}

fn repo_nota() -> RepositorioOrmMySql::<AlunoNota> {
    let sql_connection = configuration::get_mysql_string_connection();
    return RepositorioOrmMySql::<AlunoNota>::new(&sql_connection);
}
