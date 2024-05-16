use axum::{
    response::{IntoResponse, Json},
    http::StatusCode,
    extract::Path,
};

use crate::dtos::aluno_dto::AlunoDto;

use crate::model_views::{ 
    aluno_view::AlunoView,
    erro_view::ErroView
};

pub async fn index() -> impl IntoResponse {
    let alunos = vec![
        AlunoView {
            id: 1,
            nome: "Fake".to_string(),
            matricula: "Matricula Fake".to_string(),
            notas: vec![5.7, 8.9],
            situacao: "Aprovado".to_string(),
            media: 7.3,
        },
        AlunoView {
            id: 2,
            nome: "Fake 2".to_string(),
            matricula: "Matricula Fake 2".to_string(),
            notas: vec![3.7, 8.9],
            situacao: "Recuperação".to_string(),
            media: 6.3,
        }
    ];

    Json(alunos)
}

pub async fn mostrar(Path(id): Path<i32>) -> impl IntoResponse {
    if id == 1 {
        let aluno = AlunoView {
            id,
            nome: "Fake".to_string(),
            matricula: "Matricula Fake".to_string(),
            notas: vec![3.7, 8.9],
            situacao: "Recuperação".to_string(),
            media: 6.3,
        };
        Ok(Json(aluno))
    } else {
        Err((StatusCode::NOT_FOUND, Json(ErroView{ mensagem: "Aluno não encontrado".to_string() })))
    }
}

pub async fn apagar(Path(id): Path<i32>) -> impl IntoResponse {
    if id == 1 {
        println!("Usuário {} apagado", id);

        Ok( ( StatusCode::NO_CONTENT,  Json(()) ) )
    } else {
        Err((StatusCode::NOT_FOUND, Json(ErroView{ mensagem: "Aluno não encontrado".to_string() })))
    }
}

pub async fn criar(Json(aluno_dto): Json<AlunoDto>,) -> (StatusCode, Json<AlunoView>) {
    println!("Recebendo novo aluno: {:?}", aluno_dto);

    let aluno = AlunoView {
        id: 1,
        nome: "Fake".to_string(),
        matricula: "Matricula Fake".to_string(),
        notas: vec![
            3.7, 8.9
        ],
        situacao: "Recuperação".to_string(),
        media: 6.3,
    };

    (StatusCode::CREATED, Json(aluno))
}

pub async fn alterar(Path(id): Path<i32>, Json(aluno_dto): Json<AlunoDto>,) -> impl IntoResponse {
    if id != 1 {
        return Err((StatusCode::NOT_FOUND, Json(ErroView{ mensagem: "Aluno não encontrado".to_string() })));
    }
    
    println!("Recebendo aluno: {:?}", aluno_dto);

    let aluno = AlunoView {
        id: 1,
        nome: "Fake".to_string(),
        matricula: "Matricula Fake".to_string(),
        notas: vec![
            3.7, 8.9
        ],
        situacao: "Recuperação".to_string(),
        media: 6.3,
    };

    Ok( (StatusCode::OK, Json(aluno)) )
}