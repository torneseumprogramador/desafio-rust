use std::{fs::File, io::{Read, Write}};

use crate::models::aluno::Aluno;

pub struct AlunoJsonRepo {
    pub path: String
}

impl AlunoJsonRepo {
    pub fn todos(&self) -> Vec<Aluno> {
        let mut arquivo = File::open(&self.path).expect("Erro ao ler os dados no disco");

        let mut conteudo = String::new();
        arquivo.read_to_string(&mut conteudo).expect("Erro ao coverter os dados em string");

        let alunos: Vec<Aluno> = serde_json::from_str(&conteudo).expect("Erro ao converter json em obj");

        return alunos;
    }

    pub fn salvar(&self, aluno: Aluno) {
        let mut alunos = self.todos();
        alunos.push(aluno);

        let alunos_json = serde_json::to_string(&alunos).expect("Erro ao converter dados em string Json");

        let mut arquivo = File::create(&self.path).expect("Erro ao gravar arquivo json");
        arquivo.write_all(alunos_json.as_bytes()).expect("Erro ao gravar arquivo json");
    }
}