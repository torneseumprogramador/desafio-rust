enum Option<T> {
    Some(T), // algum valor
    None, // nenhum valor
}

fn main(){
    let x: i32 = 5;
    let y: Option<i32> = Some(5);

    println!("teste");
}

