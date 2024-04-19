use std::io;
use crate::ux::tela::{ limpar_tela, mostrar_mensagem, mostrar_mensagem_controlando_tempo };
use crate::models::aluno::Aluno;
use crate::models::aluno_nota::AlunoNota;
use crate::orm_desafio_v1::repositorio_orm_mysql::RepositorioOrmMySql;
use crate::config::configuration;
use std::collections::HashMap;

pub fn capturar_notas_aluno(id_aluno: &i32, nome_aluno: &String){
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
            return capturar_notas_aluno(id_aluno, nome_aluno);
        },
    };
   
    repo_nota().incluir(&AlunoNota{
        id: 0,
        aluno_id: *id_aluno,
        nota: nota
    });

    mostrar_mensagem_controlando_tempo("Nota adicionada com sucesso, vamos para a próxima nota ...", 1);

    return capturar_notas_aluno(id_aluno, nome_aluno);
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

    let aluno_id = repo().incluir(&Aluno{
        id: 0,
        nome: nome.clone(),
        matricula: matricula
    });

    let mut params = HashMap::new();
    params.insert("aluno_id".to_string(), aluno_id.to_string());
    repo_nota().apagar_where("aluno_id = :aluno_id".to_string(), &params);

    capturar_notas_aluno(&aluno_id, &nome);
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

        repo().atualizar(&Aluno{
            id: aluno.id,
            matricula: aluno.matricula,
            nome: nome.clone()
        });

        capturar_notas_aluno(&aluno.id, &nome);
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
        println!("Notas: {:?}", aluno_notas(&aluno));
        println!("Média: {:.2}", aluno_media(&aluno));
        println!("Situacao: {}", aluno_situacao(&aluno));
    }
    
    println!("\n\nDigite enter para continuar...");
    let mut continuar  = String::new();
    io::stdin().read_line(&mut continuar).unwrap();

    limpar_tela();
}

fn aluno_notas(aluno: &Aluno) -> Vec<f32> {
    let mut params = HashMap::new();
    params.insert("aluno_id".to_string(), aluno.id.to_string());
    let aluno_notas = repo_nota().where_query("aluno_id = :aluno_id".to_string(), &params);
    aluno_notas.iter().map(|an| an.nota).collect()
}

fn aluno_media(aluno: &Aluno) -> f32 {
    let notas = aluno_notas(aluno);

    let soma: f32 = notas.iter().sum();
    soma / notas.len() as f32
}

fn aluno_situacao(aluno: &Aluno) -> String {
    let media = aluno_media(aluno);

    if media >= 7.0 {
        return String::from("Aprovado")
    }
    else if media < 5.0 {
        return String::from("Reprovado")
    }

    String::from("Recuperação")
}

fn repo() -> RepositorioOrmMySql::<Aluno> {
    let sql_connection = configuration::get_mysql_string_connection();
    return RepositorioOrmMySql::<Aluno>::new(&sql_connection);
}

fn repo_nota() -> RepositorioOrmMySql::<AlunoNota> {
    let sql_connection = configuration::get_mysql_string_connection();
    return RepositorioOrmMySql::<AlunoNota>::new(&sql_connection);
}
