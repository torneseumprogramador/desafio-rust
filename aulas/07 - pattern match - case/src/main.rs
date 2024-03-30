fn mostra_dados(){
    let x = 1;
    let y = 2;
    let r = x + y;

    println!("oiii - {}", r)
}

fn main() {
    mostra_dados();

    let number: i8 = 5;

    match number {
        1 => println!("Um"),
        2 => println!("Dois"),
        3 => println!("Três"),
        _ => println!("Outro número"),
    }
}