// fn main() {
//     let s = "oláaa".to_string();
//     let s = String::from("Olá");

//     let s = String::new();
//     let s = "".to_string();

//     let mut z = String::new();
//     z += "teste";

//     println!("{}", s);
//     println!("----oi ----");
// }

// fn main() {
//     // let s = "   Hello, world!   ".trim();
//     // let s = "   Hello, world!   ".trim_end();
//     let s = "   Hello, world!   ".trim_start();

//     println!("-{}-", s);
// }

// fn main() {
//     // let s = "hello".to_uppercase();
//     let s = "HELLO".to_lowercase();
//     println!("{}", s);
// }


// fn main() {
//     let original = "Hello, world!";
//     let replaced = original.replace("world", "Rust");
//     println!("{}", replaced); // Saída: "Hello, Rust!"
// }



// use inflector::Inflector;

// fn main() {
//     let s = "hello_world".to_camel_case();
//     println!("{}", s);

//     let s = "helloWorld".to_snake_case();
//     println!("{}", s);

//     let s = "helloWorld".to_pascal_case();
//     println!("{}", s);
// }


// fn main() {
//     // Inverter caracteres de uma string
//     let s: String = "hello".chars().rev().collect();
//     println!("{}", s);
// }



// fn main() {
//     // contar quantidade de bit
//     // let quantidade: usize = "héllo".len();
//     // contar quantidade de chars
//     let quantidade: usize = "héllo".chars().count();
//     println!("Quantidade de: {}", quantidade);
// }


// fn main() {
//     let contains_substring = "hello, world".contains("world");
//     println!("existe ?: {}", contains_substring);

//     let contains_substring = String::from("hello, world").contains("worlds");
//     println!("existe ?: {}", contains_substring);
// }


// fn main() {
//     let texto = "Hello, world! Welcome to Rust programming.";
    
//     // Dividindo a string pelo espaço
//     let palavras: Vec<&str> = texto.split(' ').collect();
    
//     println!("{:?}", palavras);
// }

// fn main() {
//     // substring
//     let s = "hello";
//     let substring = &s[0..2]; // "he"

//     println!("{}", substring);
// }


// use regex::Regex;

// fn main() {
//     let email_regex = Regex::new(r"^\w+@\w+\.\w+$").unwrap();
//     let email = "example@example.com";

//     if email_regex.is_match(email) {
//         println!("{} é um email válido.", email);
//     } else {
//         println!("{} é um email inválido.", email);
//     }
// }


use regex::Regex;

fn main() {
    let phone_regex = Regex::new(r"\(\d{2}\) \d{4,5}-\d{4}").unwrap();
    let text = "O meu telefone é (12) 93333-6266. d sd ds ds ds ds dssd ";

    match phone_regex.captures(text) {
        Some(caps) => println!("Número encontrado: {}", caps.get(0).unwrap().as_str()),
        None => println!("Não foi encontrado número."),
    }
}