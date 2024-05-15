use warp::{http::StatusCode, Reply, Rejection};
use crate::dtos::aluno_dto::AlunoDto;
use crate::model_views::aluno_view::AlunoView;

pub fn index() -> impl Reply {
    let alunos = vec![
        AlunoView {
            id: 1,
            nome: "Fake".to_string(),
            matricula: "Matricula Fake".to_string(),
            notas: vec![
                5.7, 8.9
            ],
            situacao: "Aprovado".to_string(),
            media: 7.3,
        },
        AlunoView {
            id: 2,
            nome: "Fake 2".to_string(),
            matricula: "Matricula Fake 2".to_string(),
            notas: vec![
                3.7, 8.9
            ],
            situacao: "Recuperação".to_string(),
            media: 6.3,
        }
    ];

    warp::reply::json(&alunos)
}

pub async fn mostrar(id: i32) -> Result<impl warp::Reply, warp::Rejection> {
    println!("Id do aluno: {}", id);

    if id == 1 {
        let aluno = AlunoView {
            id: id,
            nome: "Fake".to_string(),
            matricula: "Matricula Fake".to_string(),
            notas: vec![
                3.7, 8.9
            ],
            situacao: "Recuperação".to_string(),
            media: 6.3,
        };

        Ok(warp::reply::json(&aluno))
    }
    else {
        Err(warp::reject::not_found())
    }
}

pub async fn criar(body: AlunoDto) -> Result<impl Reply, Rejection> {
    println!("Recebendo novo aluno: {:?}", body);

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

    Ok(warp::reply::with_status(warp::reply::json(&aluno), StatusCode::CREATED))
}