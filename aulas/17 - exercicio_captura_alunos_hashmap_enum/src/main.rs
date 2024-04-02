use std::collections::HashMap;
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

fn capturar_notas_aluno(nome_aluno: &String, notas: &mut Vec<f32>){
    println!("Digite a nota do(a) {}: (ou 'fim' para concluir)", nome_aluno);
    let mut nota_str = String::new();
    io::stdin().read_line(&mut nota_str).unwrap();

    if nota_str.trim().to_lowercase().contains("fim") {
        limpar_tela();
        return;
    }

    let nota: f32 = match nota_str.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            mostrar_mensagem("Nota inválida, digite novamente ...");
            return capturar_notas_aluno(nome_aluno, notas);
        },
    };
    notas.push(nota);

    mostrar_mensagem("Nota adicionada com sucesso, vamos para a próxima nota ...");

    return capturar_notas_aluno(nome_aluno, notas);
}

enum InfoAluno {
    Texto(String),
    Lista(Vec<f32>),
}

fn cadastrar_aluno(alunos: &mut Vec<HashMap<String, InfoAluno>> ){
    let mut nome = String::new();
    let mut matricula = String::new();
    
    println!("Digite o nome do aluno: ");
    io::stdin().read_line(&mut nome).unwrap();
    nome = nome.trim().to_string();

    println!("Digite a matrícula do aluno: ");
    io::stdin().read_line(&mut matricula).unwrap();
    matricula = matricula.trim().to_string();

    let mut notas: Vec<f32> = Vec::new();
    capturar_notas_aluno(&nome, &mut notas);

    let mut aluno:HashMap<String, InfoAluno> = HashMap::new();
    aluno.insert("nome".to_string(), InfoAluno::Texto(nome));
    aluno.insert("matricula".to_string(), InfoAluno::Texto(matricula));
    aluno.insert("notas".to_string(), InfoAluno::Lista(notas));

    alunos.push(aluno);
}

fn listar_alunos(alunos: &Vec<HashMap<String, InfoAluno>>){
    limpar_tela();
    if alunos.len() == 0 {
        mostrar_mensagem("Nenhum aluno cadastrado");
        return;
    }

    for aluno_hash in alunos.iter(){
        let nome = match aluno_hash.get("nome").unwrap() {
            InfoAluno::Texto(valor) => valor,
            _ => panic!("Valor não existente para nome"),
        };

        let matricula = match aluno_hash.get("matricula").unwrap() {
            InfoAluno::Texto(valor) => valor,
            _ => panic!("Valor não existente para nome"),
        };

        let notas = match aluno_hash.get("notas").unwrap() {
            InfoAluno::Lista(valor) => valor,
            _ => panic!("Valor não existente para nome"),
        };

        println!("{}", "-".repeat(40));
        println!("Nome: {}", nome);
        println!("Matricula: {}", matricula);
        println!("Notas: {:?}", notas);
    }
    
    println!("\n\nDigite enter para continuar...");
    let mut continuar  = String::new();
    io::stdin().read_line(&mut continuar).unwrap();

    limpar_tela();
}

fn main(){
    /*
    === Passo 1: ===

    Sua misão é contruir um menu de sistema console

    O que você deseja fazer ?
    1 - Cadastrar aluno { iniciando cadastro de aluno }
    2 - Alterar aluno  { iniciando alteracao de aluno }
    3 - Excluir aluno { iniciando exclusão de aluno }
    4 - Listar alunos { listando alunos }
    5 - Sair do programa

    === Passo 2: ====

    Agora que vc já sabe criar uma função que vc já sabe como utilizar
    um array ou tupla ou vetor

    faça a implementação da opção 1 e da opção 4
    o que colocar no cadastro de aluno
    nome, matricula, notas{vetor(f32)}



    amanha
    - vericiar a parte 2 do exercicio dos alunos
    - fazer correções sobre o exercicio
    - o que é o hashmap e onde ele entra neste contexto
    - o que é a struct e onde ela entra neste contexto
    - modulos (definição de arquitetura)
    */

    let mut alunos: Vec<HashMap<String, InfoAluno>> = Vec::new();

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
                mostrar_mensagem("Iniciando alteração de aluno");               
            },
            3 => {
                mostrar_mensagem("Iniciando exclusão de aluno ");                
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