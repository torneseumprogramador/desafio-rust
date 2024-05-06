use rocket::response::Redirect;
use rocket::form::Form;

use crate::servicos::login_servico;
use crate::dtos::login_dto::LoginDto;
use rocket::http::{Cookie, CookieJar, SameSite};
use rocket::time::Duration;  // Importando Duration do Rocket, se estiver disponível.

#[post("/logar", data = "<login_dto>")]
pub fn logar(mut cookies: &CookieJar<'_>, login_dto: Form<LoginDto>) -> Result<Redirect, Template> {
    match login_servico::logar(login_dto.into_inner()) {
        Ok(_) => {
            // TODO gravar valor do cookie com o ID com token JWT
            let user_id_token = "1";

            let mut cookie = Cookie::new("user_rocket_id", user_id_token);
            cookie.set_path("/");
            cookie.set_max_age(Duration::days(1));  // Duração de 1 dia
            cookie.set_same_site(SameSite::Lax);  // Usando SameSite do Rocket
            cookie.set_http_only(true);  // Torna o cookie acessível apenas por HTTP

            cookies.add(cookie);
            Ok(Redirect::to("/alunos"))
        },
        Err(mensagem) => {
            Err(Template::render("login/index", context! { erro: mensagem }))
        }
    }
}