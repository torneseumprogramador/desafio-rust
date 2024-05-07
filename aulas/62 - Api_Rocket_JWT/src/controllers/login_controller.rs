use crate::servicos::login_servico;
use crate::dtos::login_dto::LoginDto;
use rocket::serde::json::Json;
use crate::model_views::error_view::ErrorView;
use crate::model_views::usuario_view::UsuarioView;
use rocket::http::Status;
use rocket::response::status;
use crate::models::usuario::Usuario;
use crate::servicos::jwt_servico;

#[post("/logar", data = "<login_dto>")]
pub fn logar(login_dto: Json<LoginDto>) -> Result< status::Custom<Json<UsuarioView>>, status::Custom<Json<ErrorView>> > {
    match login_servico::logar(login_dto.into_inner()) {
        Ok(usuario) => {
            let nome_clonado = usuario.nome.clone();
            let email_clonado = usuario.email.clone();

            Ok(
                status::Custom(
                    Status::Ok,
                    Json(UsuarioView {
                        id: usuario.id,
                        nome: nome_clonado,
                        email: email_clonado,
                        token: token_jwt(&usuario)
                    })
                )
            )
        },
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

#[get("/nao_autorizado")]
pub fn unauthorized() -> status::Custom<Json<ErrorView>> {
    status::Custom(Status::Unauthorized, Json(ErrorView {
        mensagem: "Sem autorização para acessar esta área".to_string(),
    }))
}

fn token_jwt(usuario: &Usuario) -> String{
    jwt_servico::gerar(usuario)
}