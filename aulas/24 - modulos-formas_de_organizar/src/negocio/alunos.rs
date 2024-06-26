use std::io;
use crate::ux::tela::{ limpar_tela, mostrar_mensagem, mostrar_mensagem_controlando_tempo };
use crate::models::aluno::Aluno;

pub fn capturar_notas_aluno(nome_aluno: &String, notas: &mut Vec<f32>){
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
            mostrar_mensagem_controlando_tempo("Nota inválida, digite novamente ...", 1);
            return capturar_notas_aluno(nome_aluno, notas);
        },
    };
    notas.push(nota);

    mostrar_mensagem_controlando_tempo("Nota adicionada com sucesso, vamos para a próxima nota ...", 1);

    return capturar_notas_aluno(nome_aluno, notas);
}


pub fn cadastrar_aluno(alunos: &mut Vec<Aluno> ){
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

pub fn alterar_aluno(alunos: &mut Vec<Aluno> ){
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


pub fn excluir_aluno(alunos: &mut Vec<Aluno>){
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

pub fn buscar_aluno_por_matricula<'a>(matricula: &str, alunos: &'a Vec<Aluno>) -> Option<&'a Aluno> {
    for aluno in alunos.iter() {
        if aluno.matricula == matricula {
            return Some(aluno);
        }
    }
    None
}


pub fn listar_alunos(alunos: &Vec<Aluno>){
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
        println!("Média: {:.2}", aluno.media());
        println!("Situacao: {}", aluno.situacao());
    }
    
    println!("\n\nDigite enter para continuar...");
    let mut continuar  = String::new();
    io::stdin().read_line(&mut continuar).unwrap();

    limpar_tela();
}