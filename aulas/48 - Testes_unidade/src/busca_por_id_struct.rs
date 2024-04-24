#[derive(Debug, Clone, PartialEq)]
pub struct Aluno {
    pub id: i32,
    pub nome: String,
}

pub fn buscar_por_id(id: i32) -> Option<Aluno> {
    let alunos = [
        Aluno { id: 1, nome: String::from("Maria") },
        Aluno { id: 2, nome: String::from("João") },
        Aluno { id: 3, nome: String::from("Ana") },
    ];

    alunos.iter().find(|&aluno| aluno.id == id).cloned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn busca_com_id_valido() {
        // Arrange
        let id_esperado = 1;
        let nome_esperado = "Maria".to_string();
        let aluno_esperado = Aluno {
            id: id_esperado,
            nome: nome_esperado,
        };

        // Act
        let resultado = buscar_por_id(id_esperado);

        // Assert
        assert_eq!(resultado, Some(aluno_esperado));
    }


    #[test]
    fn busca_com_id_invalido() {
        // Arrange
        let id_nao_existe = 20;

        // Act
        let resultado = buscar_por_id(id_nao_existe);

        // Assert
        assert_eq!(resultado, None);
    }
}
