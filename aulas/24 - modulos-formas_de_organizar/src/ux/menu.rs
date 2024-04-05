use crate::models::aluno::Aluno;
use crate::ux::tela::{limpar_tela, mostrar_mensagem};
use std::io;
use crate::alunos::{ cadastrar_aluno, alterar_aluno, excluir_aluno, listar_alunos };

pub fn carregar(){
    let mut alunos: Vec<Aluno> = Vec::new();

    loop {
        print!("\n");
        println!("Digite uma das opção abaixo:");
        println!("-----------------------");
        println!("1. Cadastrar aluno");
        println!("2. Alterar aluno");
        println!("3. Excluir aluno");
        println!("4. Listar aluno");
        println!("5. Sair do programa");
        println!("-----------------------");

        let mut opcao = String::new();
        io::stdin().read_line(&mut opcao).expect("Falha ao ler entrada");

        let opcao: u8 = match opcao.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match opcao {
            1 => {
                cadastrar_aluno(&mut alunos);
            },
            2 => {
                alterar_aluno(&mut alunos);     
            },
            3 => {
                excluir_aluno(&mut alunos);            
            },
            4 => {
                listar_alunos(&alunos);            
            },
            5 => {
                println!("Saindo do programa...");
                break;
            },
            _ => {
                mostrar_mensagem("Opção inválida!");
            },
        }

        limpar_tela();
    }
}