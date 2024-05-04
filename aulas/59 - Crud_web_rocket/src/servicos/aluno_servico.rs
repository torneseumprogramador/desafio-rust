use crate::models::aluno::Aluno;
use crate::model_views::aluno_view::AlunoView;
use crate::models::aluno_nota::AlunoNota;
use crate::orm_desafio_v1::repositorio_orm_mysql::RepositorioOrmMySql;
use crate::config::configuration;
use std::collections::HashMap;
use crate::dtos::aluno_dto::AlunoDto;

pub fn buscar_por_id(id: i32) -> Option<AlunoView>{
    let repo = repo_aluno();
    match repo.exec_sql_to_hashmap_vec(format!("SELECT alunos.id, alunos.nome, alunos.matricula, alunos_notas.nota FROM alunos INNER JOIN alunos_notas ON alunos_notas.aluno_id = alunos.id where alunos.id = {id}").as_str()) {
        Ok(rows) => {
            let mut aluno_map: HashMap<i32, AlunoView> = HashMap::new();
            
            for row in rows {
                let id = row.get("id").and_then(|id| id.parse::<i32>().ok()).unwrap_or_default();
                let nome = row.get("nome").cloned().unwrap_or_default();
                let matricula = row.get("matricula").cloned().unwrap_or_default();
                let nota: f32 = row.get("nota").and_then(|n| n.parse().ok()).unwrap_or(0.0);

                let aluno = aluno_map.entry(id).or_insert_with(|| AlunoView {
                    id,
                    nome: nome.clone(),
                    matricula: matricula.clone(),
                    notas: Vec::new(),
                    media: 0.0,
                    situacao: String::new()
                });

                aluno.notas.push(nota);
            }
            
            for (_, mut aluno) in aluno_map {
                if !aluno.notas.is_empty() {
                    let sum: f32 = aluno.notas.iter().sum();
                    let count = aluno.notas.len() as f32;
                    aluno.media = sum / count;
                    aluno.situacao = if aluno.media < 5.0 {
                        "Reprovado".to_string()
                    } else if aluno.media < 7.0 {
                        "Recuperação".to_string()
                    } else {
                        "Aprovado".to_string()
                    }
                }

                return Some(aluno);
            }

            return None;
        },
        Err(e) => { 
            eprintln!("Error executing query: {}", e);
            return None;
        },
    }
}

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

    let aluno_id = repo_aluno().incluir(&Aluno{
        id: 0,
        nome: aluno_dto.nome,
        matricula: aluno_dto.matricula
    });

    let notas_array = string_para_array_notas(&aluno_dto.notas)?;

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

pub fn alterar(id:i32, aluno_dto: AlunoDto) -> Result<(), (String, Option<AlunoView>)>{
    match buscar_por_id(id) {
        Some(aluno) => {
            if aluno_dto.nome.is_empty() {
                return Err((String::from("O nome é obrigatório"), Some(aluno)));
            }
        
            if aluno_dto.matricula.is_empty() {
                return Err((String::from("A matricula é obrigatória"), Some(aluno)));
            }
        
            if aluno_dto.notas.is_empty() {
                return Err((String::from("Digite as notas separadas por vírgula"), Some(aluno)));
            }
        
            repo_aluno().atualizar(&Aluno{
                id: id,
                nome: aluno_dto.nome,
                matricula: aluno_dto.matricula
            });
        
            match string_para_array_notas(&aluno_dto.notas) {
                Ok(notas_array) => {
                    let repositorio_nota = repo_nota();

                    repositorio_nota.apagar_where("aluno_id = :aluno_id".to_string(), &HashMap::from([
                        ("aluno_id".to_string(), id.to_string()),
                    ]));

                    for nota in notas_array {
                        repositorio_nota.incluir(&AlunoNota {
                            id: 0,
                            aluno_id: id,
                            nota: nota
                        });
                    }
                
                    Ok(())
                }
                Err(mensagem) => {
                    return Err((mensagem, Some(aluno)));
                }
            }
        },
        None => Err((String::from("Aluno não encontrado!"), None))
    }
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

pub fn todos() -> Vec<AlunoView> {
    let mut alunos: Vec<AlunoView> = Vec::new();
    let repo = repo_aluno();

    match repo.exec_sql_to_hashmap_vec("SELECT alunos.id, alunos.nome, alunos.matricula, alunos_notas.nota FROM alunos INNER JOIN alunos_notas ON alunos_notas.aluno_id = alunos.id order by alunos.id asc") {
        Ok(rows) => {
            let mut aluno_map: HashMap<i32, AlunoView> = HashMap::new();
            
            for row in rows {
                let id = row.get("id").and_then(|id| id.parse::<i32>().ok()).unwrap_or_default();
                let nome = row.get("nome").cloned().unwrap_or_default();
                let matricula = row.get("matricula").cloned().unwrap_or_default();
                let nota: f32 = row.get("nota").and_then(|n| n.parse().ok()).unwrap_or(0.0);

                let aluno = aluno_map.entry(id).or_insert_with(|| AlunoView {
                    id,
                    nome: nome.clone(),
                    matricula: matricula.clone(),
                    notas: Vec::new(),
                    media: 0.0,
                    situacao: String::new()
                });

                aluno.notas.push(nota);
            }
            
            for (_, mut aluno) in aluno_map {
                if !aluno.notas.is_empty() {
                    let sum: f32 = aluno.notas.iter().sum();
                    let count = aluno.notas.len() as f32;
                    aluno.media = sum / count;
                    aluno.situacao = if aluno.media < 5.0 {
                        "Reprovado".to_string()
                    } else if aluno.media < 7.0 {
                        "Recuperação".to_string()
                    } else {
                        "Aprovado".to_string()
                    }
                }

                alunos.push(aluno);
            }
        },
        Err(e) => eprintln!("Error executing query: {}", e),
    }

    alunos
}

pub fn todos_com_n_mais_um() -> (Vec<Aluno>, Vec<Vec<AlunoNota>>, Vec<f32>, Vec<String>) {
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
            "Reprovado".to_string()
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
