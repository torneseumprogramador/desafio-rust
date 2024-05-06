use serde::{Serialize};

#[derive(Debug, Default, Serialize)]
pub struct AlunoView {
    pub id: i32,
    pub nome: String,
    pub matricula: String,
    pub notas: Vec<f32>,
    pub media: f32,
    pub situacao: String,
}