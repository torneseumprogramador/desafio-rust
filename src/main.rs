use std::io;
use chrono::{Utc, Datelike};
use regex::Regex;

fn main() {
    /*
    Dado que eu tenha um ano de nascimanto, e faço a subtração pelo ano atual,
    Então devo ter o valor da idade da pessoa
    */

    println!("Digite a data de seu nascimento (dd/mm/yyyy):");

    let mut data_nascimento = String::new();
    io::stdin().read_line(&mut data_nascimento).expect("Falha ao ler entrada");

    let data_regex = Regex::new(r"^\d{2}\/\d{2}\/\d{4}$").unwrap();
    if ! data_regex.is_match(&data_nascimento.trim()) {
        println!("A data que você digitou ({}) não está no formato permitido", data_nascimento.trim());
        return;
    }

    let data_split: Vec<&str> = data_nascimento.split("/").collect();

    let dia_usuario: u32 = data_split[0].trim().parse().expect("Por favor, digite um número válido!");
    let mes_usuario: u32 = data_split[1].trim().parse().expect("Por favor, digite um número válido!");
    let ano_usuario: i32 = data_split[2].trim().parse().expect("Por favor, digite um número válido!");

    let data_atual = Utc::now();
    let dia_atual: u32 = data_atual.day();
    let mes_atual: u32 = data_atual.month();
    let ano_atual: i32 = data_atual.year();

    let mut idade_usuario :u8 = (ano_atual - ano_usuario) as u8;

    if mes_usuario > mes_atual {
        idade_usuario -= 1;
    }
    else if dia_usuario > dia_atual {
        idade_usuario -= 1;
    }

    println!("A sua idade é: {}", idade_usuario);
}
