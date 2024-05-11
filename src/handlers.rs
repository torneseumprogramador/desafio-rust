use actix_web::{web, HttpResponse, Responder};
use crate::models::Message;
use crate::models::AlunoDto;
use crate::models::ErrorView;
use crate::services::AlunoServico;

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