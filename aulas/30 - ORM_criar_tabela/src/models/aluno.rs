use crate::traits::entidade::TEntidade;

pub struct Aluno {
    pub id: i32,
    pub nome: String,
    pub matricula: String,
}

impl TEntidade for Aluno {
    fn id(&self) -> i32 {
        self.id
    }

    fn campos_model(&self) -> Vec<String> {
        let mut nome_campos: Vec<String> = Vec::new();

        nome_campos.push("nome".to_string());
        nome_campos.push("matricula".to_string());

        nome_campos
    }
}