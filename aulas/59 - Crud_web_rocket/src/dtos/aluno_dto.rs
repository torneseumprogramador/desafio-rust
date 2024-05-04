
#[derive(FromForm)]
pub struct AlunoDto {
    pub nome: String,
    pub matricula: String,
    pub notas: String
}