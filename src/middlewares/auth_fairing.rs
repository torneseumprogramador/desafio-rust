use rocket::{fairing::{Fairing, Info, Kind}, Request, Data};

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
        return;

        // let request_path = request.uri().path();
        // // Lista de rotas com metch exato que não requerem autenticação
        // let open_routes = [
        //     "/login",
        //     "/",
        //     "/logar"
        // ];

        // if open_routes.contains(&request_path.as_str()) {
        //     return;
        // }
        
        // ==== TOKEN JWT ====

        // let mut cookie_valid = false;

        // if let Some(cookie_header) = request.headers().get_one("Cookie") {
        //     cookie_valid = validate_cookie(cookie_header);
        // }

        // if !cookie_valid {
        //     if let Ok(uri) = Origin::parse("/login") {
        //         request.set_uri(uri);
        //         request.set_method(Method::Get);
        //     }
        // }
    }
}

// fn validate_cookie(cookie_header: &str) -> bool {
//     println!("===== Raw Cookie Header: {} =====", cookie_header);
//     if cookie_header.contains("user_rocket_id=") {
//         // TODO pegar o valor do cookie por regex e fazer o deserialze do token JWT para o id o usuario e consultar no banco de dados se o usuário existe
//         return true;
//     }

//     false
// }
