use std::io;

fn main(){
    /*
    Regra para que eu possa validar os alunos aovivo no youtube
        usuario - repo
        didox - meu_exercicio

    Exercicio:
    Dado que agora vc conhece a forma de fazer loop no rust, crie para mim
    um programa de tabuada, onde vc irá digitar o multiplicados

    e o resultado terá que aparecer
    multiplicador = 2

    1 x 2 = 2
    2 x 2 = 4
    ....
    10 x 2 = 20
    */

    println!("Digite o valor da tabuada");

    let mut valor_tabuada: String = String::new();
    io::stdin()
        .read_line(&mut valor_tabuada)
        .expect("Falha ao ler a linha");

    let valor_tabuada: i16 = valor_tabuada.trim().parse().expect("Por favor, digite um número!");

    for multiplicador in 1..=10 {
        println!("{} X {} = {}", multiplicador, valor_tabuada, (multiplicador * valor_tabuada))
    }
}