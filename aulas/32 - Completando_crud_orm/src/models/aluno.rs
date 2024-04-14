use crate::traits::entidade::TEntidade;
use mysql::serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize, Debug)]
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

    fn from_data_row(data: HashMap<String, Value>) -> Result<Box<Self>, String> {
        let id = match data.get("id").and_then(|v| v.as_i64()) {
            Some(id) => id as i32,
            None => return Err("Id não encontrado ou inválido".to_string()),
        };
    
        let nome = match data.get("nome").and_then(|v| v.as_str()) {
            Some(nome) => nome.to_string(),
            None => return Err("Nome não encontrado ou inválido".to_string()),
        };
    
        let matricula = match data.get("matricula").and_then(|v| v.as_str()) {
            Some(matricula) => matricula.to_string(),
            None => return Err("Matricula não encontrada ou inválida".to_string()),
        };
    
        Ok(Box::new(Aluno { id, nome, matricula }))
    }
}