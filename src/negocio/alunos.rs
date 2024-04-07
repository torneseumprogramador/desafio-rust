use std::io;
use crate::ux::tela::{ limpar_tela, mostrar_mensagem, mostrar_mensagem_controlando_tempo };
use crate::models::aluno::Aluno;
// use crate::repositorios::aluno_json_repo::AlunoJsonRepo;
use crate::repositorios::aluno_mysql_repo::AlunoMySqlRepo;
use crate::config::configuration;

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

    repo().incluir(Aluno {
        nome: nome,
        matricula: matricula,
        notas: notas
    });
    
}

pub fn alterar_aluno(){
    limpar_tela();

    let alunos = repo().todos(); //TODO um método count()
    if alunos.len() == 0 { // TODO count == 0
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

        repo().alterar(Aluno{
            matricula: aluno.matricula,
            nome: nome,
            notas: notas
        })
    }

    let msg = format!("Aluno com matrícula {} não encontrado.", matricula);
    mostrar_mensagem(&msg);
}


pub fn excluir_aluno(){
    limpar_tela();
    let alunos = repo().todos(); //TODO um método count()
    if alunos.len() == 0 { // TODO count == 0
        mostrar_mensagem("Nenhum aluno cadastrado");
        return;
    }

    let mut matricula: String = String::new();

    println!("Digite a matrícula do aluno: ");
    io::stdin().read_line(&mut matricula).unwrap();
    matricula = matricula.trim().to_string();

    if let Some(aluno) = buscar_aluno_por_matricula(&matricula) {
        let msg = format!("Aluno {} excluido com sucesso.", aluno.nome);
        repo().excluir(&matricula);
        mostrar_mensagem(&msg);
    } else {
        let msg = format!("Aluno com matrícula {} não encontrado.", matricula);
        mostrar_mensagem(&msg);
    }
}

pub fn buscar_aluno_por_matricula(matricula: &str) -> Option<Aluno> {
    let alunos = &repo().todos(); //TODO Alteração no mysql para fazer um where ....

    for aluno in alunos.iter() {
        if aluno.matricula == matricula {
            return Some(aluno.clone());
        }
    }
    None
}


pub fn listar_alunos(){
    limpar_tela();
    let alunos = repo().todos(); //TODO um método count()
    if alunos.len() == 0 { // TODO count == 0
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

// fn repo() -> AlunoJsonRepo {
//     let json_file = configuration::get_json_db_alunos_path();
//     AlunoJsonRepo{ 
//         path: String::from(json_file) 
//     }
// }

fn repo() -> AlunoMySqlRepo {
    let sql_connection = configuration::get_mysql_string_connection();
    return AlunoMySqlRepo::new(&sql_connection);
}