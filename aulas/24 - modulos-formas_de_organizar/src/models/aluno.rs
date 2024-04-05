pub struct Aluno {
    pub nome: String,
    pub matricula: String,
    pub notas: Vec<f32>
}

impl Aluno {
    pub fn media(&self) -> f32 {
        let soma: f32 = self.notas.iter().sum();
        soma / self.notas.len() as f32
    }

    pub fn situacao(&self) -> String {
        let media: f32 = self.media();
        if media >= 7.0 {
            return String::from("Aprovado")
        }
        else if media < 5.0 {
            return String::from("Reprovado")
        }

        String::from("Recuperação")
    }
}