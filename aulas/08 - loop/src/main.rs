fn main() {
    let mut x = 0;
    loop {
        x += 1;

        if x == 3 {
            continue;
        }

        println!("novamente! - {}", x);

        if x > 10 {
            break;
        }
    }
}
