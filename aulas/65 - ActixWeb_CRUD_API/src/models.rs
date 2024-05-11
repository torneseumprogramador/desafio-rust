use crate::orm_desafio_v1::traits::TEntidade;
use serde::{Serialize, Deserialize};
use crate::orm_desafio_v1::create_struct_and_metadata_com_sql_methods;

create_struct_and_metadata_com_sql_methods! {
    "alunos" => Aluno {
        id: i32, "autoincrement",
        nome: String, "varchar(150)",
        matricula: String, "varchar(50)"
    }
}

create_struct_and_metadata_com_sql_methods! {
    "alunos_notas" => AlunoNota {
        id: i32, "autoincrement",
        aluno_id: i32, "int",
        nota: f32, "float",
    }
}

#[derive(Debug, Default, Serialize)]
pub struct AlunoView {
    pub id: i32,
    pub nome: String,
    pub matricula: String,
    pub notas: Vec<f32>,
    pub media: f32,
    pub situacao: String,
}

#[derive(Serialize)]
pub struct Message {
    pub mensagem: String,
}

#[derive(Serialize, Deserialize)]
pub struct AlunoDto {
    pub nome: String,
    pub matricula: String,
    pub notas: String
}

#[derive(Serialize)]
pub struct ErrorView {
    pub mensagem: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::to_string;

    #[test]
    fn test_message_serialization() {
        let message = Message {
            mensagem: String::from("Olá, mundo!"),
        };

        let serialized_message = to_string(&message).unwrap();

        assert_eq!(serialized_message, r#"{"mensagem":"Olá, mundo!"}"#);
    }
}
