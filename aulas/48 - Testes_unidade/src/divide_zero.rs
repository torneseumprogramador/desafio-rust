pub fn divide(numerator: i32, denominator: i32) -> Result<f32, String> {
    if denominator == 0 {
        Err(String::from("Erro: Divisão por zero."))
    } else {
        Ok((numerator / denominator) as f32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn teste_divisao_valida() -> Result<(), String> {
        // Arrage
        let valor_para_dividir = 8;
        let valor_divisao = 2;

        // Act
        let resultado = divide(valor_para_dividir, valor_divisao)?;

        // Assert
        assert_eq!(resultado, 4.0);

        Ok(())
    }

    #[test]
    fn teste_divisao_invalida(){
        // Arrage
        let valor_para_dividir = 8;
        let valor_divisao = 0;

        // Act
        let resultado_com_erro = divide(valor_para_dividir, valor_divisao);

        // assert
        assert!(resultado_com_erro.is_err());
    }

    #[test]
    fn teste_divisao_invalida_validando_mensagem_erro() -> Result<(), String> {
        // Arrage
        let valor_para_dividir = 8;
        let valor_divisao = 0;

        // Act
        let resultado_com_erro = divide(valor_para_dividir, valor_divisao);

        // assert
        assert!(resultado_com_erro.is_err());
        assert_eq!( resultado_com_erro, Err(String::from("Erro: Divisão por zero.")) );

        Ok(())
    }


    #[test]
    #[ignore = "Vou fazer este teste depois"]
    fn teste_divisao_invalida_validando_mensagem_erro_parte2() -> Result<(), String> {
        // Arrage
        let valor_para_dividir = 8;
        let valor_divisao = 0;

        Ok(())
    }
}