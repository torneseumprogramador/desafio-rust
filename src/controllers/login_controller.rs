use rocket_dyn_templates::{Template, context};
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
            let mut cookie = Cookie::new("user_rocket_id", "1");
            cookie.set_path("/");
            cookie.set_max_age(Duration::days(1));  // Duração de 1 dia
            cookie.set_same_site(SameSite::Lax);  // Usando SameSite do Rocket

            cookies.add(cookie);
            Ok(Redirect::to("/alunos"))
        },
        Err(mensagem) => {
            Err(Template::render("login/index", context! { erro: mensagem }))
        }
    }
}


#[get("/login")]
pub fn index() -> Template {
    Template::render("login/index", context! { erro: String::new() })
}
