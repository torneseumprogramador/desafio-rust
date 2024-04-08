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

    fn salvar(&self, alunos: &Vec<Aluno>){
        let alunos_json = serde_json::to_string(alunos).expect("Erro ao converter dados em string Json");

        let mut arquivo = File::create(&self.path).expect("Erro ao gravar arquivo json");
        arquivo.write_all(alunos_json.as_bytes()).expect("Erro ao gravar arquivo json");
    }

    pub fn incluir(&self, aluno: Aluno) {
        let mut alunos = self.todos();
        alunos.push(aluno);
        self.salvar(&alunos);
    }

    pub fn alterar(&self, aluno: Aluno) {
        let mut alunos = self.todos();

        for aluno_db in alunos.iter_mut() {
            if aluno_db.matricula == aluno.matricula {
                aluno_db.nome = aluno.nome;
                aluno_db.notas = aluno.notas;
                break;
            }
        }

        self.salvar(&alunos);
    }

    pub fn excluir(&self, matricula: &String) {
        let mut alunos = self.todos();
        alunos.retain(|a| a.matricula != *matricula);
        self.salvar(&alunos);
    }
}