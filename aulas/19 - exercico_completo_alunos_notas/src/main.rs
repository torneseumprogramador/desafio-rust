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

struct Aluno {
    nome: String,
    matricula: String,
    notas: Vec<f32>
}

fn cadastrar_aluno(alunos: &mut Vec<Aluno> ){
    limpar_tela();

    let mut nome = String::new();
    let mut matricula = String::new();

    println!("Digite a matrícula do aluno: ");
    io::stdin().read_line(&mut matricula).unwrap();
    matricula = matricula.trim().to_string();

    if let Some(aluno) = buscar_aluno_por_matricula(&matricula, alunos) {
        let msg = format!("Aluno da matricula {}, já foi cadastrado, seu nome é {}.", aluno.matricula, aluno.nome);
        mostrar_mensagem(&msg);
        return;
    }

    println!("Digite o nome do aluno: ");
    io::stdin().read_line(&mut nome).unwrap();
    nome = nome.trim().to_string();

    let mut notas: Vec<f32> = Vec::new();
    capturar_notas_aluno(&nome, &mut notas);

    alunos.push(Aluno {
        nome: nome,
        matricula: matricula,
        notas: notas
    });
}

fn alterar_aluno(alunos: &mut Vec<Aluno> ){
    limpar_tela();

    if alunos.len() == 0 {
        mostrar_mensagem("Nenhum aluno cadastrado");
        return;
    }

    let mut matricula: String = String::new();

    println!("Digite a matrícula do aluno: ");
    io::stdin().read_line(&mut matricula).unwrap();
    matricula = matricula.trim().to_string();

    for aluno in alunos.iter_mut() {
        if aluno.matricula == matricula {
            
            let mut nome = String::new();
            println!("Digite o nome do aluno: ");
            io::stdin().read_line(&mut nome).unwrap();
            nome = nome.trim().to_string();

            let mut notas: Vec<f32> = Vec::new();
            capturar_notas_aluno(&nome, &mut notas);

            aluno.nome = nome;
            aluno.notas = notas;

            return;
        }
    }

    let msg = format!("Aluno com matrícula {} não encontrado.", matricula);
    mostrar_mensagem(&msg);
}


fn excluir_aluno(alunos: &mut Vec<Aluno>){
    limpar_tela();
    if alunos.len() == 0 {
        mostrar_mensagem("Nenhum aluno cadastrado");
        return;
    }

    let mut matricula: String = String::new();

    println!("Digite a matrícula do aluno: ");
    io::stdin().read_line(&mut matricula).unwrap();
    matricula = matricula.trim().to_string();

    if let Some(aluno) = buscar_aluno_por_matricula(&matricula, alunos) {
        let msg = format!("Aluno {} excluido com sucesso.", aluno.nome);
        alunos.retain(|a| a.matricula != matricula);
        mostrar_mensagem(&msg);
    } else {
        let msg = format!("Aluno com matrícula {} não encontrado.", matricula);
        mostrar_mensagem(&msg);
    }
}

fn buscar_aluno_por_matricula<'a>(matricula: &str, alunos: &'a Vec<Aluno>) -> Option<&'a Aluno> {
    for aluno in alunos.iter() {
        if aluno.matricula == matricula {
            return Some(aluno);
        }
    }
    None
}


fn listar_alunos(alunos: &Vec<Aluno>){
    limpar_tela();
    if alunos.len() == 0 {
        mostrar_mensagem("Nenhum aluno cadastrado");
        return;
    }

    for aluno in alunos.iter(){
        println!("{}", "-".repeat(40));
        println!("Nome: {}", aluno.nome);
        println!("Matricula: {}", aluno.matricula);
        println!("Notas: {:?}", aluno.notas);
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

    === Passo 3: ====
    Agora que vc já conhece o struct, implemente os passos 2 e 3



    amanha
    - Corrigir exercicio 3
    - modulos (definição de arquitetura)
    - melhorar o exercicio - separando em modulos
    - enum (aprofundar um pouco mais)
    - hashmap (aprofundar um pouco mais)
    - struct (aprofundar um pouco mais) -> Métodos consigo fazer POO
    */

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