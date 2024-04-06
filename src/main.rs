mod models;
mod ux;
mod negocio;
mod repositorios;

use models::aluno::Aluno;
use ux::menu;


fn main() {
    // fn main(){
    /*
    amanha
    - persistencia
    */

    let repo = repositorios::aluno_json_repo::AlunoJsonRepo {
        path: "db/alunos.json".to_string()
    };


    repo.salvar(Aluno{
        nome: "Reinaldo".to_string(),
        matricula: "SP_SORO".to_string(),
        notas: vec![
            7.8, 6.0, 4.5
        ]
    });

    println!("Dados salvos");

    let alunos = repo.todos();

    for aluno in alunos.iter() {
        println!("{:?}", aluno)
    }

    // menu::carregar();

    
}