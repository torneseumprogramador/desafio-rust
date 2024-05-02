use rocket_dyn_templates::{Template, context};
use crate::servicos::aluno_servico;
use rocket::response::Redirect;

#[get("/alunos")]
pub fn index() -> Template {
    let (alunos, alunos_notas) = aluno_servico::todos();
    Template::render("alunos/index", context! { alunos: alunos, alunos_notas: alunos_notas })
}

#[post("/alunos/<id>")]
pub fn excluir(id: i32) -> Redirect {
    aluno_servico::excluir(id);
    Redirect::to(uri!(index))
}

#[get("/alunos/novo")]
pub fn novo() -> Template {
    Template::render("alunos/novo", context! { erro : String::from("") })
}

#[post("/alunos/criar")]
pub fn criar() -> Template {
    let mensagem = String::from("");
    Template::render("alunos/novo", context! { erro: mensagem })
}
