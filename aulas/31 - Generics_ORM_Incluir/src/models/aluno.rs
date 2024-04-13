use crate::traits::entidade::TEntidade;
use mysql::serde::Serialize;

#[derive(Serialize)]
pub struct Aluno {
    pub id: i32,
    pub nome: String,
    pub matricula: String,
}

impl TEntidade for Aluno {
    fn id(&self) -> i32 {
        self.id
    }

    fn campos_model() -> Vec<(String, String)> {
        vec![
            ("nome".to_string(), "varchar(150)".to_string()),
            ("matricula".to_string(), "varchar(50)".to_string())
        ]
    }
}