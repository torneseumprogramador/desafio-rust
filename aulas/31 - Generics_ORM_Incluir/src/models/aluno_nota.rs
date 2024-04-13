use crate::traits::entidade::TEntidade;
use mysql::serde::Serialize;

#[derive(Serialize)]
pub struct AlunoNota {
    pub id: i32,
    pub aluno_id: i32,
    pub nota: f32
}

impl TEntidade for AlunoNota {
    fn id(&self) -> i32 {
        self.id
    }

    fn campos_model() -> Vec<(String, String)> {
        vec![
            ("aluno_id".to_string(), "int".to_string()),
            ("nota".to_string(), "float".to_string())
        ]
    }
}