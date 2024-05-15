use serde::Serialize;

#[derive(Serialize)]
pub struct AlunoView {
    pub id: i32,
    pub nome: String,
    pub matricula: String,
    pub notas: Vec<f32>,
    pub situacao: String,
    pub media: f32
}