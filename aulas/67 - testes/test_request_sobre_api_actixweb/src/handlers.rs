use actix_web::{web, HttpResponse, Responder};
use crate::models::Message;
use crate::models::AlunoDto;
use crate::models::LoginDto;
use crate::models::UsuarioView;
use crate::models::ErrorView;
use crate::services::AlunoServico;
use crate::services::LoginServico;
use crate::jwt::{ create_token, Claims };
use crate::config;
use std::time::{SystemTime, UNIX_EPOCH};

pub async fn index() -> impl Responder {
    HttpResponse::Ok().json(Message { mensagem: "Olá pessoal em formato JSON".to_string() })
}

pub async fn alunos_index() -> impl Responder {
    let alunos = AlunoServico::todos();
    HttpResponse::Ok().json(alunos)
}

pub async fn alunos_show(path: web::Path<(i32,)>) -> impl Responder {
    let id: i32 = path.0;
    match AlunoServico::buscar_por_id(id) {
        Some(aluno) => HttpResponse::Ok().json(aluno),
        None => HttpResponse::NotFound().finish()
    }
}

pub async fn alunos_criar(aluno_dto: web::Json<AlunoDto>) -> impl Responder {
    match AlunoServico::incluir(aluno_dto.into_inner()) {
        Ok(aluno_view) => HttpResponse::Created().json(aluno_view),
        Err(mensagem) => HttpResponse::BadRequest().json(ErrorView { mensagem: mensagem })
    }
}

pub async fn alunos_atualizar(path: web::Path<(i32,)>, aluno_dto: web::Json<AlunoDto>) -> impl Responder {
    let id = path.0;
    match AlunoServico::alterar(id, aluno_dto.into_inner()) {
        Ok(aluno_view) => HttpResponse::Ok().json(aluno_view),
        Err((mensagem, _)) => HttpResponse::BadRequest().json(ErrorView { mensagem: mensagem })
    }
}

pub async fn alunos_apagar(path: web::Path<(i32,)>) -> impl Responder {
    let id = path.0;
    AlunoServico::excluir(id);
    HttpResponse::NoContent().finish()
}

pub async fn logar(login_dto:  web::Json<LoginDto>) -> impl Responder {
    match LoginServico::logar(login_dto.into_inner()) {
        Ok(usuario) => {
            let nome_clonado = usuario.nome.clone();
            let email_clonado = usuario.email.clone();

            let expiration = (SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs() + 60) * 60 * 24; // Token expira em 1 dia
    
            let claims = Claims {
                sub: usuario.id.to_string(),
                exp: expiration as usize,
            };
    
            let config = config::load_config().expect("Failed to load configuration.");
            let jwt_secret = config.jwt.secret.clone();
    
            match create_token(claims, &jwt_secret) {
                Ok(token) => HttpResponse::Ok().json(UsuarioView {
                    id: usuario.id,
                    nome: nome_clonado,
                    email: email_clonado,
                    token: token
                }),
                Err(_) => return HttpResponse::BadRequest().json(Message { 
                    mensagem: "Erro ao gerar token".to_string() 
                }),
            }
        },
        Err(mensagem) => {
            HttpResponse::BadRequest().json(Message { 
                mensagem: mensagem
            })
        }
    }
}