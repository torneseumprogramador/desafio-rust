use crate::traits::entidade::TEntidade;
use mysql::serde::{ Serialize, Deserialize };
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
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

    fn from_data_row(data: HashMap<String, Value>) -> Result<Box<Self>, String> {
        let id = match data.get("id").and_then(|v| v.as_i64()) {
            Some(id) => id as i32,
            None => return Err("Id não encontrado ou inválido".to_string()),
        };

        let aluno_id = match data.get("aluno_id").and_then(|v| v.as_i64()) {
            Some(aluno_id) => aluno_id as i32,
            None => return Err("Aluno_id não encontrado ou inválido".to_string()),
        };

        let nota = match data.get("nota").and_then(|v| v.as_f64()) {
            Some(nota) => nota as f32,
            None => return Err("Nota não encontrado ou inválido".to_string()),
        };
    
        Ok(Box::new(AlunoNota { id, aluno_id, nota }))
    }
}