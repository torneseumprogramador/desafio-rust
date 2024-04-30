#[macro_use] extern crate rocket;
use maud::html;
use rocket::response::{self, Responder, Response};
use rocket::http::ContentType;
use rocket::request::Request;

fn home() -> String {
    let nome = "Alunos";
    let markup = html! {
        doctype;
        html {
            head {
                title { "Sobre" }
            }
            body {
                h1 { "Olá, está é a pagina de sobre, mandei o parametro = " (nome) "!" }
                p { 
                    "Este é um exemplo simples com Maud. "
                    b { "Está em negrito" }
                }
            }
        }
    };

    markup.into_string()
}

struct HtmlContent(String);

impl<'r> Responder<'r, 'static> for HtmlContent {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        Response::build()
            .sized_body(self.0.len(), std::io::Cursor::new(self.0))
            .header(ContentType::HTML)
            .ok()
    }
}

#[get("/")]
fn index() -> HtmlContent {
    HtmlContent(home())
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
}
