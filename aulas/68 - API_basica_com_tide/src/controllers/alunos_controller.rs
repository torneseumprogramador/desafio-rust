use tide::{Request, Response, Result};
use crate::dtos::aluno_dto::AlunoDto;
use crate::model_views::aluno_view::AlunoView;
use serde_json::to_string;

pub async fn criar(mut _req: Request<()>) -> Result {
    let aluno_dto: AlunoDto = _req.body_json().await?;
    println!("::: Salvando o Aluno Dto {:?} :::", aluno_dto);

    let aluno_view = AlunoView {
        id: 1,
        nome: "Fake".to_string(),
        matricula: "Matricula Fake".to_string(),
        notas: vec![
            5.7, 8.9
        ],
        situacao: "Aprovado".to_string(),
        media: 7.3,
    };

    let message = to_string(&aluno_view)?;

    Ok(Response::builder(201)
        .body(message)
        .content_type(tide::http::mime::JSON)
        .build())
}

pub async fn mostrar(mut _req: Request<()>) -> Result {
    let id = _req.param("id")?.to_string();
    println!("::: Id passado {} :::", id);

    let aluno_view = AlunoView {
        id: 1,
        nome: "Fake".to_string(),
        matricula: "Matricula Fake".to_string(),
        notas: vec![
            5.7, 8.9
        ],
        situacao: "Aprovado".to_string(),
        media: 7.3,
    };

    let message = to_string(&aluno_view)?;

    Ok(Response::builder(200)
        .body(message)
        .content_type(tide::http::mime::JSON)
        .build())
}