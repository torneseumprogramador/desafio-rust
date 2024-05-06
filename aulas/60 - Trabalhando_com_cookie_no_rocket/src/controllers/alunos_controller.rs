use rocket_dyn_templates::{Template, context};
use crate::servicos::aluno_servico;
use rocket::response::Redirect;
use crate::dtos::aluno_dto::AlunoDto;
use crate::models::aluno::Aluno;
use rocket::form::Form;

#[get("/alunos")]
pub fn index() -> Template {
    let alunos = aluno_servico::todos();
    Template::render("alunos/index", context! { alunos: alunos })
}

#[post("/alunos/<id>")]
pub fn excluir(id: i32) -> Redirect {
    aluno_servico::excluir(id);
    Redirect::to(uri!(index))
}

#[get("/alunos/<id>")]
pub fn editar(id: i32) -> Result<Template, Redirect> {
    match aluno_servico::buscar_por_id(id) {
        Some(aluno) => {
            let notas_com_virgula = aluno.notas.iter().map(|num| num.to_string()).collect::<Vec<String>>().join(", ");
            Ok(Template::render("alunos/editar", context! { 
                erro : String::from(""),
                aluno: aluno,
                notas_com_virgula: notas_com_virgula
            }))
        },
        None => Err( Redirect::to(uri!(index)) )
    }
}

#[post("/alunos/<id>/alterar", data = "<aluno_dto>")]
pub fn alterar(id: i32, aluno_dto: Form<AlunoDto>) -> Result<Redirect, Template> {
    match aluno_servico::alterar(id, aluno_dto.into_inner()) {
        Ok(_) => Ok(Redirect::to(uri!(index))),
        Err((mensagem, aluno_option)) => {
            match aluno_option {
                Some(aluno) => {
                    let notas_com_virgula = aluno.notas.iter().map(|num| num.to_string()).collect::<Vec<String>>().join(", ");
                    Err(Template::render("alunos/editar", context! { 
                        erro : mensagem,
                        aluno: aluno,
                        notas_com_virgula: notas_com_virgula
                    }))
                },
                None => Err(Template::render("alunos/editar", context! { 
                    erro : mensagem,
                    aluno: Aluno::default(),
                    notas_com_virgula: String::new()
                }))
            }
        }
    }
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
