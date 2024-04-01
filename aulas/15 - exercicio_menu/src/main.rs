use std::io;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

fn limpar_tela() {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
                .args(&["/C", "cls"])
                .status()
                .unwrap();
    } else {
        Command::new("clear")
                .status()
                .unwrap();
    }
}

fn esperar(tempo:u64){
    sleep(Duration::from_secs(tempo)); 
}

fn mostrar_mensagem(mensagem: &str){
    limpar_tela();
    println!("{}", mensagem);
    esperar(2);
}

fn main(){
    /*
    Passo 1:

    Sua misão é contruir um menu de sistema console

    O que você deseja fazer ?
    1 - Cadastrar aluno { iniciando cadastro de aluno }
    2 - Alterar aluno  { iniciando alteracao de aluno }
    3 - Excluir aluno { iniciando exclusão de aluno }
    2 - Listar alunos { listando alunos }
    3 - Sair do programa
    */


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
                mostrar_mensagem("Iniciando cadastro de aluno ");
            },
            2 => {
                mostrar_mensagem("Iniciando alteração de aluno");               
            },
            3 => {
                mostrar_mensagem("Iniciando exclusão de aluno ");                
            },
            4 => {
                mostrar_mensagem("listando alunos ");                
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