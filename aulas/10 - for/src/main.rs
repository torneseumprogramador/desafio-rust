fn main(){
    for number in 1..=4 { // de de 1 a 4
        println!("Número: {}", number);
    }

    println!("--------");

    for number in 1..4 { // de 1 a < 4
        println!("Número: {}", number);
    }
}