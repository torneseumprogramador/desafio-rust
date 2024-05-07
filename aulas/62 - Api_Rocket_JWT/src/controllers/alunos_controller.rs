use crate::servicos::aluno_servico;
use rocket::serde::json::Json;
use crate::model_views::aluno_view::AlunoView;
use crate::model_views::error_view::ErrorView;
use rocket::http::Status;
use rocket::response::status;
use crate::dtos::aluno_dto::AlunoDto;

#[get("/alunos")]
pub fn index() -> Json<Vec<AlunoView>> {
    let alunos = aluno_servico::todos();
    Json(alunos)
}

#[get("/alunos/<id>")]
pub fn show(id: i32) -> status::Custom<Json<Option<AlunoView>>> {
    match aluno_servico::buscar_por_id(id) {
        Some(aluno) => status::Custom(
            Status::Ok,
            Json(Some(aluno))
        ),
        None => status::Custom(
            Status::NotFound,
            Json(None)
        )
    }
}

#[post("/alunos", data = "<aluno_dto>")]
pub fn criar(aluno_dto: Json<AlunoDto>) -> Result< status::Custom<Json<AlunoView>>, status::Custom<Json<ErrorView>> > {
    match aluno_servico::incluir(aluno_dto.into_inner()) {
        Ok(aluno_view) => Ok(
            status::Custom(
                Status::Created,
                Json(aluno_view)
            )
        ),
        Err(mensagem) => {
            Err(
                status::Custom(
                    Status::BadRequest,
                    Json(ErrorView { mensagem: mensagem })
                )
            )
        }
    }
}

#[delete("/alunos/<id>")]
pub fn excluir(id: i32) ->  status::Custom<Json<()>> {
    aluno_servico::excluir(id);
    status::Custom(Status::NoContent, Json(()))
}

#[put("/alunos/<id>", data = "<aluno_dto>")]
pub fn alterar(id: i32, aluno_dto: Json<AlunoDto>) -> Result< status::Custom<Json<AlunoView>>, status::Custom<Json<ErrorView>> > {
    match aluno_servico::alterar(id, aluno_dto.into_inner()) {
        Ok(aluno_view) => Ok(
            status::Custom(
                Status::Ok,
                Json(aluno_view)
            )
        ),
        Err((mensagem, _)) => {
            Err(
                status::Custom(
                    Status::BadRequest,
                    Json(ErrorView { mensagem: mensagem })
                )
            )
        }
    }
}


