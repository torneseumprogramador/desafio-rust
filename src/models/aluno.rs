use crate::orm_desafio_v1::create_struct_and_metadata_com_sql_methods;

create_struct_and_metadata_com_sql_methods! {
    "alunos" => Aluno {
        id: i32, "autoincrement",
        nome: String, "varchar(150)",
        matricula: String, "varchar(50)",
        notas: String, "text",
    }
}

impl Aluno {
    pub fn get_notas(&self) -> Vec<f32> {
        serde_json::from_str(&self.notas).unwrap()
    }

    pub fn set_notas(&mut self, notas_param: &Vec<f32>) {
        self.notas = serde_json::to_string(notas_param).unwrap();
    }

    pub fn media(&self) -> f32 {
        let soma: f32 = self.get_notas().iter().sum();
        soma / self.get_notas().len() as f32
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