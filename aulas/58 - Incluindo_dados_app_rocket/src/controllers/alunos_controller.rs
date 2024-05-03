use rocket_dyn_templates::{Template, context};
use crate::servicos::aluno_servico;
use rocket::response::Redirect;
use crate::dtos::aluno_dto::AlunoDto;
use rocket::form::Form;

#[get("/alunos")]
pub fn index() -> Template {
    let (alunos, alunos_notas, alunos_media, alunos_situacao) = aluno_servico::todos();
    Template::render("alunos/index", context! { alunos: alunos, alunos_notas: alunos_notas, alunos_media: alunos_media, alunos_situacao: alunos_situacao })
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

#[post("/alunos/criar", data = "<aluno_dto>")]
pub fn criar(aluno_dto: Form<AlunoDto>) -> Result<Redirect, Template> {
    match aluno_servico::incluir(aluno_dto.into_inner()) {
        Ok(_) => Ok(Redirect::to(uri!(index))),
        Err(mensagem) => {
            Err(Template::render("alunos/novo", context! { erro : mensagem }))
        }
    }
}
