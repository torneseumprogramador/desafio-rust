use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct AlunoView {
    pub id: i32,
    pub nome: String,
    pub matricula: String,
    pub notas: Vec<f32>,
    pub situacao: String,
    pub media: f32
}