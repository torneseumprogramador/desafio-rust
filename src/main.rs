// fn main() {
//     // memória stack (variáveis do tipo copy no rust)
//     let x: i32 = 4;
//     let y: i32 = x; // copia de dados 

//     println!("o valor de x é {} - Referencia {:p}", x, &x);
//     println!("o valor de y é {} - Referencia {:p}", y, &y);
// }



// fn main() {
//     // memória stack (variáveis do tipo copy no rust)
//     let x: i32 = 4;
//     let y: &i32 = &x; // copia de dados 

//     println!("o valor de x é {} - Referencia {:p}", x, &x);
//     println!("o valor de y é {} - Referencia {:p}", y, y);

//     println!("------------");

//     let yy = *y; // deconstrutor
//     println!("o valor de x é {} - Referencia {:p}", x, &x);
//     println!("o valor de yy é {} - Referencia {:p}", yy, &yy);
// }



// fn main() {
//     // memória stack (variáveis do tipo copy no rust)
//     let x: i32 = 4; // owner
//     let y: &i32 = &x; // referencia de dados (y aponta para o mesmo local que x, porém o dono é o x, se encerrar o x encerra o y )

//     println!("o valor de x é {} - Referencia {:p}", x, &x);
//     println!("o valor de y é {} - Referencia {:p}", y, y);
// }



// fn main() {
//     let mut x: i32 = 4; // Declare x como mutável
//     let y: &i32 = &x;

//     println!("O valor de x é {}, {:p}", x, &x);
//     println!("O valor de y é {}, {:p}", y, y);

//     // Modifique x para invalidar y
//     // println!("O valor de y é {}", y);
//     x = 42; // modificando o owner

//     // Agora, y se tornou uma referência inválida
//     // Tentar imprimir y resultará em um erro de tempo de compilação
//     println!("O valor de y é {}, {:p}", y, y);
// }



// fn main() {
//     // memória stack (variáveis do tipo copy no rust)
//     let x: i32 = 4;
//     let y: &i32 = &x; // copia de dados 

//     println!("Valor: {}, Endereço de memória: {:p}", x, &x);

//     imprime_valor(&x);
//     imprime_valor(y);
// }

// fn imprime_valor(valor: &i32){
//     println!("Valor: {}, Endereço de memória: {:p}", valor, valor);
// }




// fn main() {
//     // memória stack (variáveis do tipo copy no rust)
//     let mut x: i32 = 4;

//     imprime_valor(&x);
// }

// fn imprime_valor(valor: &i32){
//     valor += 1; // não pode porque tenho imudabilidade nas referencias
//     println!("Valor {}, Endereço de memória de y: {:p}", valor, valor);
// }



// fn main() {
//     // memória stack (variáveis do tipo copy no rust)
//     let mut x: i32 = 4;
//     x = 5;

//     let y: &i32 = &x; // copia de dados 

//     println!("Valor: {}, Endereço de memória: {:p}", x, &x);

//     imprime_valor(&mut x);
//     // imprime_valor(y);

//     x = 7;

//     println!("Valor: {}, Endereço de memória: {:p}", x, &x);
// }

// fn imprime_valor(valor: &mut i32){
//     *valor = 2;
//     println!("Valor: {}, Endereço de memória: {:p}", valor, valor);
// }




// fn main() {
//     let mut x: i32 = 4;
//     println!("[Original] - Valor de x após as modificações: {} - referencia: {:p}", x, &x);

//     imprime_valor(&mut x); // Passando uma referência mutável para x
    
//     println!("[Original] - Valor de x após as modificações: {} - referencia: {:p}", x, &x);

//     imprime_valor(&mut x); // Passando uma referência mutável para x

//     println!("[Original] - Valor de x após as modificações: {} - referencia: {:p}", x, &x);
// }

// fn imprime_valor(valor: &mut i32) {
//     *valor += 1; // Modificando o valor referenciado por valor utilizando um reborrowing
//     // O compilador pode mover a variável temporariamente para uma localização diferente na memória durante a referência mutável. 
//     // O objetivo é evitar possíveis problemas de aliasing e garantir a segurança das referências mutáveis
//     println!("[reborrowing] - Valor referenciado por valor: {} - referencia: {:p}", valor, &valor);
// }



// fn main() {
//     // variáveis na memória heap

//     // não é uma variável by copy
//     let s1: String = String::from("Olá"); // s1 possui a propriedade da String
//     let s2 = s1; //.clone(); // A propriedade é transferida de s1 para s2 (Borrowing)

//     // Isso causa um erro, porque s1 não é mais válido após a transferência
//     println!("s1: {} - referencia: {:p}", s1, &s1);

//     // s2 é válido e pode ser usado
//     println!("s2: {} - referencia: {:p}", s2, &s2);
// }



// fn main() {
//     // variáveis na memória heap

//     let s1 = String::from("Olá"); // s1 possui a propriedade da String
//     let s2 = s1.clone(); // s2 recebe uma cópia profunda (clone) de s1

//     // Isso causa um erro, porque s1 não é mais válido após a transferência
//     println!("s1: {} - referencia: {:p}", s1, &s1);

//     // s2 é válido e pode ser usado
//     println!("s2: {} - referencia: {:p}", s2, &s2);
// }




// fn main() {
//     // variáveis na memória heap

//     let s1 = String::from("Olá"); // s1 possui a propriedade da String

//     // Isso causa um erro, porque s1 não é mais válido após a transferência
//     println!("Antes da transferência:");
//     print_string(s1);
//     print_string(s1); // o s1 não é mais valido e nem o s da função print_string
// }

// fn print_string(s: String) {
//     println!("Valor da String: {} - referencia: {:p}", s, &s);
// }




// fn main() {
//     // variáveis na memória heap

//     let s1 = String::from("Olá"); // s1 possui a propriedade da String

//     println!("Antes da transferência:");
//     print_string(&s1);
//     print_string(&s1)
// }

// fn print_string(s: &String) {
//     println!("Valor da String: {} - referencia: {:p}", s, &s);
// }



// fn main() {
//     // Exemplo com String
//     let s1 = String::from("Olá, mundo!"); // s1 é uma String alocada na heap
//     let s2 = s1.clone(); // Clonando a String s1 para criar s2

//     println!("String s1: {} - referencia: {:p}", s1, &s1);
//     println!("String s2: {} - referencia: {:p}", s2, &s2);

//     // Exemplo com &str
//     let s3: &str = "Olá, mundo!"; // s3 é um &str (slice de string)
//     let s4: &str = s3; // s4 é uma referência para o mesmo &str

//     println!("&str s3: {} - referencia: {:p}", s3, s3);
//     println!("&str s4: {} - referencia: {:p}", s4, s4);
// }




// fn main() {
//     // Exemplo com String
//     let mut s1 = String::from("Olá, mundo!"); // s1 é uma String alocada na heap
//     s1 += " - teste";

//     let s2 = s1.clone(); // Clonando a String s1 para criar s2

//     println!("String s1: {} - referencia: {:p}", s1, &s1);
//     println!("String s2: {} - referencia: {:p}", s2, &s2);

//     // Exemplo com &str
//     let mut s3: &str = "Olá, mundo!"; // s3 é um &str (slice de string)
//     s3 += " - teste"; // ilegal


//     let mut s3: String = "Olá, mundo!".to_string(); // s3 é um &str (slice de string)
//     s3 += " - teste"; // legal

//     let xxx = format!("{} - teste", s3); // legal
//     println!("String s2: {} - referencia: {:p}", xxx, &xxx);
// }
    


// fn main() {
//     // Convertendo String em &str usando as_str()
//     let s1 = String::from("Olá, mundo!");
//     let reference_to_s1: &str = s1.as_str();
    
//     println!("s1 (String): {} - referencia: {:p}", s1, &s1);
//     println!("s1 (referência &str): {} - referencia: {:p}", reference_to_s1, reference_to_s1);
    
//     // Convertendo String em &str fazendo uma referência
//     let s2 = String::from("Rust é incrível!");
//     let reference_to_s2: &str = &s2;

//     println!("s2 (String): {} - referencia: {:p}", s2, &s2);
//     println!("s2 (referência &str): {} - referencia: {:p}", reference_to_s2, reference_to_s2);
// }



// fn main(){
//     let mut s = String::from("olá");

//     // s = s + ", mundo!"; // push_str() adiciona um literal à String
//     // s += ", mundo!"; // push_str() adiciona um literal à String
//     s.push_str(", mundo!"); // push_str() adiciona um literal à String

//     println!("{}", s); // Isso vai exibir `olá, mundo!`
// }



// fn main() {
//     let mut x = 5;
//     manda_referencia(&mut x);

//     println!("{}", x)
// }

// fn manda_referencia(x: &mut i32){
//     *x += 1;
//     println!("{}", x)
// }




// fn main() {
//     let x = 5;
//     println!("{} - {:p}", x, &x);
    
//     manda_referencia(x);

//     println!("{} - {:p}", x, &x)
// }

// fn manda_referencia(mut x: i32){
//     x += 1;
//     println!("{} - {:p}", x, &x)
// }



// fn main() {
//     let mut x: i32 = 5;
//     x = manda_via_copia(x);
    
//     println!("{}", x)
// }

// fn manda_via_copia(x: i32) -> i32{
//     x + 1
// }



fn main() {
    let s1 = String::from("téxto");

    let tamanho = calcula_tamanho(&s1);

    println!("O tamanho de '{}' é {}.", s1, tamanho);
}

fn calcula_tamanho(s: &String) -> usize {
    // s.len() // conta a quantidade de bit
    s.chars().count() // conta a quantidade de char
}