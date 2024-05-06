use rocket_dyn_templates::{Template, context};

#[get("/")]
pub fn index() -> Template {
    Template::render("home/index", context! { 
        titulo: "Meu Projeto Rocket com Templates" 
    })
}

#[get("/sobre")]
pub fn sobre() -> Template {
    Template::render("home/sobre", context! {})
}