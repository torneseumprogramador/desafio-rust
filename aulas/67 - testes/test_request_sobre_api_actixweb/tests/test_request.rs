use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct Login {
    email: String,
    senha: String,
}

#[derive(Deserialize)]
struct AuthResponse {
    token: String,
}

async fn get_token() -> String {
    let client = reqwest::Client::new();
    let login_details = Login {
        email: "danilo@teste.com".to_string(),
        senha: "123456".to_string(),
    };

    let res = client.post("http://localhost:8080/logar")
        .json(&login_details)
        .send()
        .await.expect("Falha ao fazer login");

    if res.status().is_success() {
        let auth_response: AuthResponse = res.json().await.expect("Fala ao retornar os dados");
        auth_response.token
    } else {
        String::new()
    }
}

#[cfg(test)]
mod tests {
    use super::get_token;

    #[tokio::test]
    async fn home() {
        let client = reqwest::Client::new();
        let res = client.get("http://localhost:8080/")
            .send()
            .await
            .expect("Failed to send request");
        assert!(res.status().is_success());
        let body = res.text().await.expect("Failed to read response text");
        assert_eq!("{\"mensagem\":\"Olá pessoal em formato JSON\"}", body);
    }

    #[tokio::test]
    async fn buscar_aluno_sem_token() {
        let client = reqwest::Client::new();
        let res = client.get("http://localhost:8080/alunos")
            .send()
            .await
            .expect("Failed to send request");
        assert!(res.status().is_client_error());

        let body = res.text().await.expect("Failed to read response text");
        assert_eq!("Token obrigatório", body);
    }


    use serde::{Serialize, Deserialize};

    #[derive(Debug, Default, Serialize, Deserialize)]
    pub struct AlunoView {
        pub id: i32,
        pub nome: String,
        pub matricula: String,
        pub notas: Vec<f32>,
        pub media: f32,
        pub situacao: String,
    }

    #[tokio::test]
    async fn buscar_aluno_com_token() {
        // Arrange
        let client = reqwest::Client::new();
        let token = get_token().await;

        // Act
        let res = client.get("http://localhost:8080/alunos")
            .bearer_auth(&token)
            .send()
            .await
            .expect("Failed to send request");

        // Assert
        assert!(res.status().is_success());

        // Arrange & Act
        let alunos: Vec<AlunoView> = res.json().await.expect("Failed to parse JSON");
        
        // Assert
        assert!(alunos.len() > 0);
        assert_eq!(alunos[0].id, 1);
        assert_eq!(alunos[0].nome, "Anderson Martins");
    }

    #[derive(Serialize, Deserialize, Default)]
    pub struct AlunoDto {
        pub nome: String,
        pub matricula: String,
        pub notas: String
    }

    #[tokio::test]
    async fn criar_aluno_sem_token() {
        let client = reqwest::Client::new();

        let aluno_dto = AlunoDto::default();
    
        let res = client.post("http://localhost:8080/alunos")
            .json(&aluno_dto)
            .send()
            .await.expect("Falha ao tentar criar aluno");
            
        assert!(res.status().is_client_error());

        let body = res.text().await.expect("Failed to read response text");
        assert_eq!("Token obrigatório", body);
    }

    #[tokio::test]
    async fn criar_aluno_com_token() {
        // Arrange
        let client = reqwest::Client::new();

        let token = get_token().await;

        let aluno_dto = AlunoDto {
            nome: String::from("Caue"),
            matricula: String::from("CE44"),
            notas: String::from("7.9, 5.9")
        };

        // Act
        let res = client.post("http://localhost:8080/alunos")
            .bearer_auth(&token)
            .json(&aluno_dto)
            .send()
            .await.expect("Falha ao tentar criar aluno");

        // Assert
        assert!(res.status().is_success());

        // Arrange & Act
        let aluno_view: AlunoView = res.json().await.expect("Failed to parse JSON");
        
        // Assert
        assert!(aluno_view.id > 0);
        assert_eq!(aluno_view.nome, "Caue");
        assert_eq!(aluno_view.matricula, "CE44");
        assert_eq!(aluno_view.notas[0], 7.9);
    }
    
}