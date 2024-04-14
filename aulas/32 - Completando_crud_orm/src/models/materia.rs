use crate::traits::entidade::TEntidade;
use mysql::serde::{ Serialize, Deserialize };
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
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

    fn from_data_row(data: HashMap<String, Value>) -> Result<Box<Self>, String> {
        let id = match data.get("id").and_then(|v| v.as_i64()) {
            Some(id) => id as i32,
            None => return Err("Id não encontrado ou inválido".to_string()),
        };
    
        let titulo = match data.get("titulo").and_then(|v| v.as_str()) {
            Some(titulo) => titulo.to_string(),
            None => return Err("Titulo não encontrada ou inválida".to_string()),
        };
    
        let descricao = match data.get("descricao").and_then(|v| v.as_str()) {
            Some(descricao) => descricao.to_string(),
            None => return Err("Descrição não encontrado ou inválido".to_string()),
        };
    
        Ok(Box::new(Materia { id, titulo, descricao }))
    }
}