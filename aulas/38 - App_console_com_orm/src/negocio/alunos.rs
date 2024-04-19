use std::io;
use crate::ux::tela::{ limpar_tela, mostrar_mensagem, mostrar_mensagem_controlando_tempo };
use crate::models::aluno::Aluno;
use crate::orm_desafio_v1::repositorio_orm_mysql::RepositorioOrmMySql;
use crate::config::configuration;
use std::collections::HashMap;

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


pub fn cadastrar_aluno(){
    limpar_tela();

    let mut nome = String::new();
    let mut matricula = String::new();

    println!("Digite a matrícula do aluno: ");
    io::stdin().read_line(&mut matricula).unwrap();
    matricula = matricula.trim().to_string();

    if let Some(aluno) = buscar_aluno_por_matricula(&matricula) {
        let msg = format!("Aluno da matricula {}, já foi cadastrado, seu nome é {}.", aluno.matricula, aluno.nome);
        mostrar_mensagem(&msg);
        return;
    }

    println!("Digite o nome do aluno: ");
    io::stdin().read_line(&mut nome).unwrap();
    nome = nome.trim().to_string();

    let mut notas: Vec<f32> = Vec::new();
    capturar_notas_aluno(&nome, &mut notas);

    let mut aluno = Aluno::default();
    aluno.nome = nome;
    aluno.matricula = matricula;
    aluno.set_notas(&notas);

    repo().incluir(&aluno);
}

pub fn alterar_aluno(){
    limpar_tela();

    if repo().count() == 0 {
        mostrar_mensagem("Nenhum aluno cadastrado");
        return;
    }

    let mut matricula: String = String::new();

    println!("Digite a matrícula do aluno: ");
    io::stdin().read_line(&mut matricula).unwrap();
    matricula = matricula.trim().to_string();

    if let Some(aluno) = buscar_aluno_por_matricula(&matricula) {
        let mut nome = String::new();
        println!("Digite o nome do aluno: ");
        io::stdin().read_line(&mut nome).unwrap();
        nome = nome.trim().to_string();

        let mut notas: Vec<f32> = Vec::new();
        capturar_notas_aluno(&nome, &mut notas);

        let mut aluno_updade = Aluno::default();
        aluno_updade.id = aluno.id;
        aluno_updade.matricula = aluno.matricula;
        aluno_updade.nome = nome;
        aluno_updade.set_notas(&notas);

        repo().atualizar(&aluno_updade)
    }

    let msg = format!("Aluno com matrícula {} não encontrado.", matricula);
    mostrar_mensagem(&msg);
}


pub fn excluir_aluno(){
    limpar_tela();
    if repo().count() == 0 {
        mostrar_mensagem("Nenhum aluno cadastrado");
        return;
    }

    let mut matricula: String = String::new();

    println!("Digite a matrícula do aluno: ");
    io::stdin().read_line(&mut matricula).unwrap();
    matricula = matricula.trim().to_string();

    if let Some(aluno) = buscar_aluno_por_matricula(&matricula) {
        let msg = format!("Aluno {} excluido com sucesso.", aluno.nome);
        repo().apagar_por_id(aluno.id);
        mostrar_mensagem(&msg);
    } else {
        let msg = format!("Aluno com matrícula {} não encontrado.", matricula);
        mostrar_mensagem(&msg);
    }
}

pub fn buscar_aluno_por_matricula(matricula: &str) -> Option<Aluno> {
    let mut params = HashMap::new();
    params.insert("matricula".to_string(), matricula.to_string());
    let alunos = repo().where_query("matricula = :matricula".to_string(), &params);
    
    if alunos.len() > 0 {
        return Some(alunos[0].clone());
    }

    None
}


pub fn listar_alunos(){
    limpar_tela();
    let repo = repo();
    
    if repo.count() == 0 {
        mostrar_mensagem("Nenhum aluno cadastrado");
        return;
    }

    for aluno in repo.todos().iter(){
        println!("{}", "-".repeat(40));
        println!("Nome: {}", aluno.nome);
        println!("Matricula: {}", aluno.matricula);
        println!("Notas: {:?}", aluno.get_notas());
        println!("Média: {:.2}", aluno.media());
        println!("Situacao: {}", aluno.situacao());
    }
    
    println!("\n\nDigite enter para continuar...");
    let mut continuar  = String::new();
    io::stdin().read_line(&mut continuar).unwrap();

    limpar_tela();
}

fn repo() -> RepositorioOrmMySql::<Aluno> {
    let sql_connection = configuration::get_mysql_string_connection();
    return RepositorioOrmMySql::<Aluno>::new(&sql_connection);
}