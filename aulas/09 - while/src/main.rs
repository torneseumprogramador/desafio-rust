fn main() {
    let mut x = 1;
    while x <= 20 {

        if x >= 10 && x <= 15 { 
            x += 1;
            continue; 
        }

        if x > 10 { break; }

        println!("novamente! {}", x);

        x += 1;
    }
}