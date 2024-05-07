use rocket::{http::Method, Request, Data, fairing::{Fairing, Info, Kind}, http::uri::Origin};
use crate::servicos::jwt_servico;

pub struct AuthFairing;

#[rocket::async_trait]
impl Fairing for AuthFairing {
    fn info(&self) -> Info {
        Info {
            name: "Authentication Fairing",
            kind: Kind::Request,
        }
    }

    async fn on_request(&self, request: &mut Request<'_>, _: &mut Data<'_>) {

        let request_path = request.uri().path();
        // Lista de rotas com metch exato que não requerem autenticação
        let open_routes = [
            "/",
            "/logar"
        ];

        if open_routes.contains(&request_path.as_str()) {
            return;
        }

        let token_valid = request.headers().get_one("Authorization")
            .and_then(|header| header.strip_prefix("Bearer "))
            .map(|token| jwt_servico::validar(token))
            .unwrap_or(false);

        if !token_valid {
            if let Ok(uri) = Origin::parse("/nao_autorizado") {
                request.set_uri(uri);
                request.set_method(Method::Get);
            }
        }
        
    }
}