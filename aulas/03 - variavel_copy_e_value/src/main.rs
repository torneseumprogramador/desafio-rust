
// fn main() {
//     let x: i8 = 5;
//     let y: i8 = x; // estou fazendo uma cópia do valor de x para y (tipos primitivos = armazenam na memória stack)

//     println!("x: {}, y: {}", x, y)
// }

fn main() {
    let x: String = String::from("Alunos de Rust");
    // let y: String = x; // é um codigo inválido pois ele invalida o x ao mudar de dono
    // let y: String = x.clone(); // não tem tanta performance, pois duplica dados na memória heap
    let y: &String = &x; // aponto para o ponteiro da variável x, tendo uma melhor performance por não duplicar dados na memória heap

    
    println!("x: {}, y: {}", x, y)
}