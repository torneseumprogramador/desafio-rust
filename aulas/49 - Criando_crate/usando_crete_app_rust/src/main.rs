extern crate validador_cpf; 
use validador_cpf as validador;
use std::io;

fn main() {
    println!("Digite um CPF");

    let mut cpf = String::new();

    match io::stdin().read_line(&mut cpf) {
        Ok(_) => {
            println!("Você digitou: {}", cpf.trim());
        },
        Err(e) => {
            println!("Erro ao ler entrada: {}", e);
        }
    }

    let validado: bool = validador::validar_cpf(cpf.as_str());

    if validado {
        println!("O CPF é valido")
    } else {
        println!("O CPF é inválido")
    }
}