use crate::models::Aluno;
use crate::models::AlunoView;
use crate::models::AlunoNota;
use crate::orm_desafio_v1::repositorio_orm_mysql::RepositorioOrmMySql;
use crate::models::AlunoDto;
use crate::config;
use std::collections::HashMap;
use crate::models::Usuario;
use bcrypt::{hash, DEFAULT_COST, verify, BcryptError};
use crate::models::LoginDto;

pub struct AlunoServico;
impl AlunoServico {
    pub fn buscar_por_id(id: i32) -> Option<AlunoView>{
        let repo = AlunoServico::repo_aluno();
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

    pub fn incluir(aluno_dto: AlunoDto) -> Result<AlunoView, String> {
        if aluno_dto.nome.is_empty() {
            return Err(String::from("O nome é obrigatório"));
        }

        if aluno_dto.matricula.is_empty() {
            return Err(String::from("A matricula é obrigatória"));
        }

        if aluno_dto.notas.is_empty() {
            return Err(String::from("Digite as notas separadas por vírgula"));
        }

        let nome_clonado = aluno_dto.nome.clone();
        let matricula_clonada = aluno_dto.matricula.clone();

        let aluno_id = AlunoServico::repo_aluno().incluir(&Aluno {
            id: 0,
            nome: aluno_dto.nome,
            matricula: aluno_dto.matricula
        });

        let notas_array = AlunoServico::string_para_array_notas(&aluno_dto.notas)?;

        let repositorio_nota = AlunoServico::repo_nota();
        for nota in &notas_array {
            repositorio_nota.incluir(&AlunoNota {
                id: 0,
                aluno_id: aluno_id,
                nota: *nota
            });
        }

        let (media, situacao) = AlunoServico::get_media_situacao(&notas_array);

        Ok(AlunoView {
            id: aluno_id,
            nome: nome_clonado,
            matricula: matricula_clonada,
            notas: notas_array,
            media: media,
            situacao: situacao,
        })
    }

    pub fn alterar(id:i32, aluno_dto: AlunoDto) -> Result<AlunoView, (String, Option<AlunoView>)>{
        match AlunoServico::buscar_por_id(id) {
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

                let nome_clonado = aluno_dto.nome.clone();
                let matricula_clonada = aluno_dto.matricula.clone();
            
                AlunoServico::repo_aluno().atualizar(&Aluno{
                    id: id,
                    nome: aluno_dto.nome,
                    matricula: aluno_dto.matricula
                });
            
                match AlunoServico::string_para_array_notas(&aluno_dto.notas) {
                    Ok(notas_array) => {
                        let repositorio_nota = AlunoServico::repo_nota();

                        repositorio_nota.apagar_where("aluno_id = :aluno_id".to_string(), &HashMap::from([
                            ("aluno_id".to_string(), id.to_string()),
                        ]));

                        for nota in &notas_array {
                            repositorio_nota.incluir(&AlunoNota {
                                id: 0,
                                aluno_id: id,
                                nota: *nota
                            });
                        }

                        let (media, situacao) = AlunoServico::get_media_situacao(&notas_array);
                    
                        Ok(AlunoView {
                            id: id,
                            nome: nome_clonado,
                            matricula: matricula_clonada,
                            notas: notas_array,
                            media: media,
                            situacao: situacao,
                        })
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
        AlunoServico::repo_nota().apagar_where("aluno_id = :aluno_id".to_string(), &HashMap::from([
            ("aluno_id".to_string(), id.to_string()),
        ]));

        AlunoServico::repo_aluno().apagar_por_id(id);
    }

    pub fn todos() -> Vec<AlunoView> {
        let mut alunos: Vec<AlunoView> = Vec::new();
        let repo = AlunoServico::repo_aluno();

        match repo.exec_sql_to_hashmap_vec("SELECT alunos.id, alunos.nome, alunos.matricula, alunos_notas.nota FROM alunos INNER JOIN alunos_notas ON alunos_notas.aluno_id = alunos.id ORDER BY alunos.nome ASC") {
            Ok(rows) => {
                let mut temp_map: HashMap<i32, (usize, AlunoView)> = HashMap::new();
                let mut index = 0;

                for row in rows {
                    let id = row.get("id").and_then(|id| id.parse::<i32>().ok()).unwrap_or_default();
                    let nome = row.get("nome").cloned().unwrap_or_default();
                    let matricula = row.get("matricula").cloned().unwrap_or_default();
                    let nota: f32 = row.get("nota").and_then(|n| n.parse().ok()).unwrap_or(0.0);

                    let entry = temp_map.entry(id).or_insert_with(|| (index, AlunoView {
                        id,
                        nome: nome.clone(),
                        matricula: matricula.clone(),
                        notas: Vec::new(),
                        media: 0.0,
                        situacao: String::new()
                    }));

                    entry.1.notas.push(nota); // pegando a segunda posição da tupla
                    index += 1;
                }

                let mut sorted_alunos: Vec<_> = temp_map.into_iter().map(|(_, v)| v).collect();
                sorted_alunos.sort_by_key(|(idx, _)| *idx);

                for (_, mut aluno) in sorted_alunos {
                    let (media, situacao) = AlunoServico::get_media_situacao(&aluno.notas);
                    aluno.media = media;
                    aluno.situacao = situacao;
                    alunos.push(aluno);
                }
            },
            Err(e) => eprintln!("Error executing query: {}", e),
        }

        alunos
    }

    fn get_media_situacao(notas: &Vec<f32>) -> (f32, String) {
        let sum: f32 = notas.iter().sum();
        let count = notas.len() as f32;
        let media:f32 = sum / count;
        let situacao:String = if media < 5.0 {
            "Reprovado".to_string()
        } else if media < 7.0 {
            "Recuperação".to_string()
        } else {
            "Aprovado".to_string()
        };

        (media, situacao)
    }

    fn repo_aluno() -> RepositorioOrmMySql::<Aluno> {
        let sql_connection = config::get_mysql_string_connection();
        return RepositorioOrmMySql::<Aluno>::new(&sql_connection);
    }

    fn repo_nota() -> RepositorioOrmMySql::<AlunoNota> {
        let sql_connection = config::get_mysql_string_connection();
        return RepositorioOrmMySql::<AlunoNota>::new(&sql_connection);
    }
}

pub struct UsuarioServico;
impl UsuarioServico {
    pub fn incluir(usuario: Usuario) -> Result<i32, String> {
        if usuario.nome.is_empty() {
            return Err(String::from("O nome é obrigatório"));
        }

        if usuario.email.is_empty() {
            return Err(String::from("A email é obrigatória"));
        }

        if usuario.senha.is_empty() {
            return Err(String::from("A senha é obrigatória"));
        }

        let senha_cripto = UsuarioServico::criptografa_senha(&usuario.senha);

        let usuario_senha_criptografada = Usuario {
            id: 0,
            nome: usuario.nome,
            email: usuario.email,
            senha: senha_cripto
        };

        let aluno_id = UsuarioServico::repo().incluir(&usuario_senha_criptografada);
        Ok(aluno_id)
    }

    fn criptografa_senha(senha: &str) -> String {
        hash(senha, DEFAULT_COST).unwrap()
    }

    fn repo() -> RepositorioOrmMySql::<Usuario> {
        let sql_connection = config::get_mysql_string_connection();
        return RepositorioOrmMySql::<Usuario>::new(&sql_connection);
    }
}


pub struct LoginServico;
impl LoginServico {
    pub fn logar(login_dto: LoginDto) -> Result<Usuario, String>{
        if login_dto.email.is_empty() {
            return Err(String::from("O email é obrigatório"));
        }

        if login_dto.senha.is_empty() {
            return Err(String::from("A senha é obrigatória"));
        }
        
        let sql_connection = config::get_mysql_string_connection();
        let repo = RepositorioOrmMySql::<Usuario>::new(&sql_connection);

        let usuarios = repo.where_query("email = :email".to_string(), &HashMap::from([
            ("email".to_string(), login_dto.email),
        ]));

        if usuarios.len() == 0  {
            return Err(String::from("Email ou senha inválidos"));
        }

        let usuario = usuarios[0].clone();
        match LoginServico::senha_valida(&usuario.senha, &login_dto.senha) {
            Ok(validado) => {
                if validado {
                    Ok(usuario)
                }
                else {
                    Err(String::from("Email ou senha inválidos"))
                }
            },
            Err(_) => Err(String::from("Email ou senha inválidos"))
        }
    }

    fn senha_valida(senha_db: &str, senha_dto: &str) -> Result<bool, BcryptError> {
        verify(senha_dto, senha_db)
    }
}
