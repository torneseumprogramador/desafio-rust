use actix_web::{HttpResponse, Responder};
use crate::models::Message;
use crate::models::Aluno;

pub async fn index() -> impl Responder {
    HttpResponse::Ok().json(Message { mensagem: "Olá pessoal em formato JSON".to_string() })
}

pub async fn alunos_index() -> impl Responder {
    let alunos = vec![
        Aluno { 
            id: 1,
            nome: String::from("Gerusa"),
            matricula: String::from("GE445"),
            notas: vec![
                6.7,
                8.8
            ],
        }
    ];
    
    HttpResponse::Ok().json(alunos)
}