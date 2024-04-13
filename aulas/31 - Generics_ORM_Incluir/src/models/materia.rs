use crate::traits::entidade::TEntidade;
use mysql::serde::Serialize;

#[derive(Serialize)]
pub struct Materia {
    pub id: i32,
    pub titulo: String,
    pub descricao: String,
}

impl TEntidade for Materia {
    fn id(&self) -> i32 {
        self.id
    }

    fn campos_model() -> Vec<(String, String)> {
        vec![
            ("titulo".to_string(), "varchar(150)".to_string()),
            ("descricao".to_string(), "text".to_string())
        ]
    }
}