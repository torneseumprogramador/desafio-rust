use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AlunoDto {
    pub nome: String,
    pub matricula: String,
    pub notas: String
}