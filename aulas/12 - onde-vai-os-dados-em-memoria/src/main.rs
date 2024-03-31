// memória static
static mut xxx: i32 = 5;
const UMA_VARIAVEL:&str = "teste";

fn main() {
    println!("{}", UMA_VARIAVEL);

    mostra_xx();

    // memória stack
    let x: i32 = 4;

    // memória heap
    let s: String = String::from("Danilo Aparecido sd ds dsds ds ds  ds ds ds ");

    println!("{} - ", xxx)
}

fn mostra_xx(){
    // println!("{}", UMA_VARIAVEL2);
    println!("{}", UMA_VARIAVEL);
    println!("{} - ", xxx)
}