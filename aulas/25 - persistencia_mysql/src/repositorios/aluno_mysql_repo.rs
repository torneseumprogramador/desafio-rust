use mysql::{Pool, PooledConn, params};
use mysql::prelude::*;
use crate::models::aluno::Aluno;

pub struct AlunoMySqlRepo {
    pool: Pool,
}

impl AlunoMySqlRepo {
    pub fn new(str_connection: &str) -> Self {
        AlunoMySqlRepo {
            pool: Pool::new(str_connection).unwrap(),
        }
    }

    fn get_conn(&self) -> PooledConn {
        self.pool.get_conn().unwrap()
    }

    pub fn todos(&self) -> Vec<Aluno> {
        let mut conn = self.get_conn();
        let alunos: Vec<Aluno> = conn
            .query_map(
                "SELECT nome, matricula, notas FROM alunos;",
                |(nome, matricula, notas): (String, String, String)| {
                    Aluno {
                        nome,
                        matricula,
                        notas: notas.split(',').filter_map(|n| n.parse().ok()).collect(),
                    }
                },
            )
            .unwrap();

        alunos
    }

    pub fn incluir(&self, aluno: Aluno) {
        let mut conn = self.get_conn();
        conn.exec_drop(
            "INSERT INTO alunos (nome, matricula, notas) VALUES (:nome, :matricula, :notas)",
            params! {
                "nome" => aluno.nome,
                "matricula" => aluno.matricula,
                "notas" => aluno.notas.iter().map(ToString::to_string).collect::<Vec<_>>().join(","),
            },
        )
        .unwrap();
    }

    pub fn alterar(&self, aluno: Aluno) {
        let mut conn = self.get_conn();
        conn.exec_drop(
            "UPDATE alunos SET nome=:nome, notas=:notas WHERE matricula=:matricula",
            params! {
                "nome" => aluno.nome,
                "matricula" => aluno.matricula,
                "notas" => aluno.notas.iter().map(ToString::to_string).collect::<Vec<_>>().join(","),
            },
        )
        .unwrap();
    }

    pub fn excluir(&self, matricula: &String) {
        let mut conn = self.get_conn();
        conn.exec_drop(
            "DELETE FROM alunos WHERE matricula=:matricula",
            params! {
                "matricula" => matricula,
            },
        )
        .unwrap();
    }
}
