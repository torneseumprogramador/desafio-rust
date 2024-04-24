pub fn validar_cnpj(cnpj: &str) -> bool {
    let digits: Vec<u8> = cnpj
        .chars()
        .filter(|c| c.is_digit(10))
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();

    if digits.len() != 14 || digits.iter().all(|&d| d == digits[0]) {
        return false;
    }

    let calculate_digit = |slice: &[u8], weights: &[usize]| -> u8 {
        let sum: usize = slice.iter()
            .zip(weights.iter())
            .map(|(&digit, &weight)| digit as usize * weight)
            .sum();

        let mod_result = sum % 11;
        (if mod_result < 2 { 0 } else { 11 - mod_result }) as u8
    };

    let first_weights: [usize; 12] = [5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];
    let second_weights: [usize; 13] = [6, 5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];

    let first_digit = calculate_digit(&digits[..12], &first_weights);
    let second_digit = calculate_digit(&digits[..13], &second_weights);

    first_digit == digits[12] && second_digit == digits[13]
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testa_cnpj_valido(){
        // AAA

        // Arrange
        let cnpj_validao = "38.724.190/0001-62";

        // Act
        let resultado = validar_cnpj(cnpj_validao);

        // Assert
        assert_eq!(resultado, true);
    }

    #[test]
    fn testa_cnpj_invalido(){
        // AAA

        // Arrange
        let cnpj_validao = "38.724.190/0001-61";

        // Act
        let resultado = validar_cnpj(cnpj_validao);

        // Assert
        assert_eq!(resultado, false);
    }

    #[test]
    fn testa_cnpj_valido_com_mascara(){
        assert_eq!(validar_cnpj("38.724.190/0001-62"), true);
    }

    #[test]
    fn testa_cnpj_valido_sem_mascara(){
        assert_eq!(validar_cnpj("38724190000162"), true);
    }

    #[test]
    fn testa_cnpj_invalido_com_digitos_incorretos(){
        assert_eq!(validar_cnpj("38.724.190/0001-61"), false);
    }

    #[test]
    fn testa_cnpj_com_caracteres_nao_numericos(){
        assert_eq!(validar_cnpj("38.724.190/0001-6x"), false);
    }

    #[test]
    fn testa_cnpj_com_todos_digitos_iguais(){
        assert_eq!(validar_cnpj("11.111.111/1111-11"), false);
    }

    #[test]
    fn testa_cnpj_com_comprimento_incorreto(){
        assert_eq!(validar_cnpj("3872419000016"), false); // Menos dígitos
        assert_eq!(validar_cnpj("38.724.190/0001-62123"), false); // Mais dígitos
    }

    #[test]
    fn testa_cnpj_vazio(){
        assert_eq!(validar_cnpj(""), false);
    }
}