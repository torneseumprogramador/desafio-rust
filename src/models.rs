use serde::{Serialize};

#[derive(Serialize)]
pub struct Message {
    pub mensagem: String,
}

#[derive(Serialize)]
pub struct Aluno {
    pub id: i32,
    pub nome: String,
    pub matricula: String,
    pub notas: Vec<f32>
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
