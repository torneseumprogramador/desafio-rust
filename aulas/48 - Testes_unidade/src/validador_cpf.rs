pub fn validar_cpf(cpf: &str) -> bool {
    let cpf: Vec<u8> = cpf
        .chars()
        .filter(|c| c.is_digit(10))
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();

    if cpf.len() != 11 || cpf.iter().all(|&digit| digit == cpf[0]) {
        return false;
    }

    let mut sum1 = 0;
    let mut sum2 = 0;

    for i in 0..9 {
        sum1 += (cpf[i] as usize) * (10 - i);
        sum2 += (cpf[i] as usize) * (11 - i);
    }

    sum2 += cpf[9] as usize * 2;

    let digit1 = (sum1 * 10) % 11 % 10;
    let digit2 = (sum2 * 10) % 11 % 10;

    cpf[9] == digit1 as u8 && cpf[10] == digit2 as u8
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testa_cpf_valido(){
        assert!(validar_cpf("973.465.310-51"));
    }

    #[test]
    fn testa_cpf_invalido(){
        assert_eq!(validar_cpf("973.465.310-52"), false);
    }

    #[test]
    fn testa_cpf_valido_sem_mascara(){
        assert_eq!(validar_cpf("97346531051"), true);
    }

    #[test]
    fn testa_cpf_invalido_sem_mascara(){
        assert_eq!(validar_cpf("97346531052"), false);
    }

    #[test]
    fn testa_cpf_valido_com_mascara_invalida(){
        assert_eq!(validar_cpf("973465-310-51"), true);
    }

    #[test]
    fn testa_cpf_invalido_com_mascara_invalida(){
        assert_eq!(validar_cpf("973465-310-52"), false);
    }

    #[test]
    fn testa_cpf_vazio(){
        assert_eq!(validar_cpf(""), false);
    }

    #[test]
    fn testa_cpf_incompleto(){
        assert_eq!(validar_cpf("97346531"), false);
    }

    #[test]
    fn testa_cpf_com_numero_sequencial_1(){
        assert_eq!(validar_cpf("111.111.111-11"), false);
    }

    #[test]
    fn testa_cpf_com_numero_sequencial_1_sem_mascara(){
        assert_eq!(validar_cpf("11111111111"), false);
    }

    #[test]
    fn testa_cpf_com_numero_sequencial_2_sem_mascara(){
        assert_eq!(validar_cpf("22222222222"), false);
    }
}