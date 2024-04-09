// trait Animal {
//     fn speak(&self);
// }

// struct Dog;

// impl Animal for Dog {
//     fn speak(&self) {
//         println!("Woof!");
//     }
// }

// struct Cat;

// impl Animal for Cat {
//     fn speak(&self) {
//         println!("Meow!");
//     }
// }


// struct Galinha;

// impl Animal for Galinha {
//     fn speak(&self) {
//         println!("Cocoricó");
//     }
// }

// impl Galinha {
//     fn bota(&self){
//         println!("Vai botar ovos");
//     }
// }


// fn speak(animal: &impl Animal){
// // fn speak(animal: &dyn Animal){ // é até demais para esta ocasição
//     animal.speak();
// }

// fn main() {
//     let dog = Dog;
//     let cat = Cat;
//     let galinha = Galinha;

//     speak(&dog);
//     speak(&cat);
//     speak(&galinha);
// }




//// ==== Como resolver um problema de stratagy em design pattern ====
// /// ======= Problema =======
// enum TipoPessoa{
//     Fisica,
//     Juridica,
//     Generica
// }

// struct Pessoa{
//     id: u32,
//     nome: String,
//     documento: String,
//     tipo: TipoPessoa
// }

// fn mostra_pessoa(pessoa: Pessoa){
//     println!("Id: {}", pessoa.id);
//     println!("Nome: {}", pessoa.nome);
//     println!("Documento: {}", pessoa.documento);
//     match pessoa.tipo {
//         TipoPessoa::Fisica => {
//             println!("Tipo: Fisica");
//         },
//         TipoPessoa::Juridica => {
//             println!("Tipo: Juridica");
//         },
//         TipoPessoa::Generica => {
//             println!("Tipo: Generica");
//         }
//     }
// }

// fn main(){
//     let pessoa_fisica = Pessoa {
//         id: 1,
//         nome: "Danilo".to_string(),
//         documento: "222.222.222-11".to_string(),
//         tipo: TipoPessoa::Fisica
//     };

//     let pessoa_juridica = Pessoa {
//         id: 1,
//         nome: "C&C".to_string(),
//         documento: "22.222.666/0001-22".to_string(),
//         tipo: TipoPessoa::Juridica
//     };

//     let pessoa_juridica_generica = Pessoa {
//         id: 1,
//         nome: "C&C - Generica".to_string(),
//         documento: "22.222.666/0001-21".to_string(),
//         tipo: TipoPessoa::Generica
//     };

//     mostra_pessoa(pessoa_fisica);
//     mostra_pessoa(pessoa_juridica);
//     mostra_pessoa(pessoa_juridica_generica);
// }

//// ====== Solução =======
// trait Pessoa{
//     fn id(&self) -> u32;
//     fn nome(&self) -> &String;
//     fn documento(&self) -> &String;
//     fn tipo(&self) -> String;
// }

// struct PessoaFisica{
//     id: u32,
//     nome: String,
//     cpf: String
// }
// impl Pessoa for PessoaFisica {
//     fn id(&self) -> u32 {
//         self.id
//     }
//     fn nome(&self) -> &String {
//         &self.nome
//     }
//     fn documento(&self) -> &String{
//         &self.cpf
//     }
//     fn tipo(&self) -> String{
//         String::from("Fisica")
//     }
// }

// struct PessoaJuridica{
//     id: u32,
//     nome: String,
//     cnpj: String
// }
// impl Pessoa for PessoaJuridica {
//     fn id(&self) -> u32 {
//         self.id
//     }
//     fn nome(&self) -> &String {
//         &self.nome
//     }
//     fn documento(&self) -> &String{
//         &self.cnpj
//     }
//     fn tipo(&self) -> String{
//         String::from("Juridica")
//     }
// }

// struct PessoaGenerica{
//     id: u32,
//     nome: String,
//     docx: String
// }
// impl Pessoa for PessoaGenerica {
//     fn id(&self) -> u32 {
//         self.id
//     }
//     fn nome(&self) -> &String {
//         &self.nome
//     }
//     fn documento(&self) -> &String{
//         &self.docx
//     }
//     fn tipo(&self) -> String{
//         String::from("Generica")
//     }
// }


// struct PessoaGenerica2{
//     id: u32,
//     nome: String,
//     docx: String
// }
// impl Pessoa for PessoaGenerica2 {
//     fn id(&self) -> u32 {
//         self.id
//     }
//     fn nome(&self) -> &String {
//         &self.nome
//     }
//     fn documento(&self) -> &String{
//         &self.docx
//     }
//     fn tipo(&self) -> String{
//         String::from("Generica")
//     }
// }

// fn mostra_pessoa(pessoa: &dyn Pessoa){
//     println!("Id: {}", pessoa.id());
//     println!("Nome: {}", pessoa.nome());
//     println!("Documento: {}", pessoa.documento());
//     println!("Tipo: {}", pessoa.tipo());
// }

// fn main(){
//     let pessoa_fisica = PessoaFisica {
//         id: 1,
//         nome: "Danilo".to_string(),
//         cpf: "222.222.222-11".to_string()
//     };

//     let pessoa_juridica = PessoaJuridica {
//         id: 1,
//         nome: "C&C".to_string(),
//         cnpj: "22.222.666/0001-22".to_string()
//     };

//     let pessoa_juridica_generica = PessoaGenerica {
//         id: 1,
//         nome: "C&C - Generica".to_string(),
//         docx: "22.222.666/0001-21".to_string()
//     };

//     let pessoa_juridica_generica2 = PessoaGenerica2 {
//         id: 1,
//         nome: "C&C - Generica 2".to_string(),
//         docx: "22.222.666/0001-21".to_string()
//     };

//     mostra_pessoa(&pessoa_fisica);
//     mostra_pessoa(&pessoa_juridica);
//     mostra_pessoa(&pessoa_juridica_generica);
//     mostra_pessoa(&pessoa_juridica_generica2);
// }





// /// ======= Problema =======
// enum Cargo{
//     Gerencia,
//     Estoque,
//     Marketing,
//     Venda,
// }

// struct Funcionario{
//     id: u32,
//     nome: String,
//     salario: f32,
//     cargo: Cargo
// }

// impl Funcionario{
//     fn salario_mais_comissao(&self) -> f32 {
//         let porcentagem = match self.cargo {
//             Cargo::Gerencia => 15.0,
//             Cargo::Marketing => 20.0,
//             Cargo::Venda => 30.0,
//             Cargo::Estoque => 5.0
//         };
        
//         self.salario + (self.salario * porcentagem / 100.0)
//     }
// }

// fn main(){
//     let gerente = Funcionario{
//         id: 1,
//         nome: "Geraldo".to_string(),
//         salario: 1000.0,
//         cargo: Cargo::Venda
//     };

//     println!("Salario é: {}", gerente.salario);
//     println!("Salario + Comissão é: {}", gerente.salario_mais_comissao());
// }



// /// ======= Solução com Generics =======

// trait Cargo {
//     fn porcentagem(&self) -> f32;
// }

// struct Gerencia;
// impl Cargo for Gerencia {
//     fn porcentagem(&self) -> f32 {
//         15.0
//     }
// }
// struct Estoque;
// impl Cargo for Estoque {
//     fn porcentagem(&self) -> f32 {
//         5.0
//     }
// }
// struct Venda;
// impl Cargo for Venda {
//     fn porcentagem(&self) -> f32 {
//         30.0
//     }
// }
// struct Marketing;
// impl Cargo for Marketing {
//     fn porcentagem(&self) -> f32 {
//         20.0
//     }
// }

// struct Funcionario<C: Cargo> {
//     id: u32,
//     nome: String,
//     salario: f32,
//     cargo: C,
// }

// impl<C: Cargo> Funcionario<C> {
//     fn salario_mais_comissao(&self) -> f32 {
//         self.salario + (self.salario * self.cargo.porcentagem() / 100.0)
//     }
// }

// fn main() {
//     let gerente = Funcionario {
//         id: 1,
//         nome: "Geraldo".to_string(),
//         salario: 1000.0,
//         cargo: Marketing{},
//     };

//     println!("Salario é: {}", gerente.salario);
//     println!("Salario + Comissão é: {}", gerente.salario_mais_comissao());
// }


//// ===== Solução sem Generics =====

// trait Cargo {
//     fn porcentagem(&self) -> f32;
// }

// struct Gerencia;
// impl Cargo for Gerencia {
//     fn porcentagem(&self) -> f32 {
//         15.0
//     }
// }
// struct Estoque;
// impl Cargo for Estoque {
//     fn porcentagem(&self) -> f32 {
//         5.0
//     }
// }
// struct Venda;
// impl Cargo for Venda {
//     fn porcentagem(&self) -> f32 {
//         30.0
//     }
// }
// struct Marketing;
// impl Cargo for Marketing {
//     fn porcentagem(&self) -> f32 {
//         20.0
//     }
// }

// struct Funcionario {
//     id: u32,
//     nome: String,
//     salario: f32,
//     cargo: Box<dyn Cargo>,
// }

// impl Funcionario {
//     fn new(id: u32, nome: String, salario: f32, cargo: Box<dyn Cargo>) -> Self {
//         Funcionario { id, nome, salario, cargo }
//     }

//     fn salario_mais_comissao(&self) -> f32 {
//         self.salario + (self.salario * self.cargo.porcentagem() / 100.0)
//     }
// }

// fn main() {
//     let gerente = Funcionario::new(
//         1,
//         "Geraldo".to_string(),
//         1000.0,
//         Box::new(Estoque{})
//     );

//     println!("Salario é: {}", gerente.salario);
//     println!("Salario + Comissão é: {}", gerente.salario_mais_comissao());
// }


//// ==== Trait + Tratamento de erro =====
///////// =========== Error Types Customizados com Trait Debug ============
use regex::Regex;
use std::error::Error;
use std::fmt;


#[derive(Debug)]
enum ValidationError {
    EmptyName(String),
    NonUniqueName(String),
    InvalidFormat(String),
}

// Implementando Display para o nosso erro customizado
impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

// Para que ValidationError possa ser tratado como um erro
impl Error for ValidationError {}

#[derive(Debug)]
struct ErroTamanho {
    mensagem: String
}
impl fmt::Display for ErroTamanho {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl Error for ErroTamanho {}

fn validar_nome(nome: &str, nomes_existentes: &[&str]) -> Result<(), Box<dyn Error>> { // onde não posso utilizar Box<impl Error> porque tenho 2 tipos de classe de erro
    // Validar se o nome não está vazio
    if nome.is_empty() {
        return Err(Box::new(ValidationError::EmptyName("O nome não pode ser vazio".to_string())));
    } else if nomes_existentes.contains(&nome) {
        return Err(Box::new(ValidationError::NonUniqueName("O nome deve ser unico".to_string())));
    }

    // Validar o formato do nome com regex
    let regex = Regex::new(r"^[a-zA-Z\s]+$").unwrap();
    // Exemplos de Strings Válidas na REGEX
    //     "Alice"
    //     "Bob Smith"
    //     "a b c"
    //     "Z"

    if !regex.is_match(nome) {
        return Err(Box::new(ValidationError::InvalidFormat("O nome não está no padrão permitido".to_string())));
    }


    if nome.trim().len() < 5 {
        return Err(Box::new(ErroTamanho{ mensagem: "Nome precisa ser maior ou igual a 5".to_string() }));
    }

    Ok(())
}

fn main() {
    let nomes_existentes = vec!["Alice", "Bob"];
    match validar_nome("Bob", &nomes_existentes) {
        Ok(_) => println!("Nome válido"),
        Err(e) => println!("Erro de validação: {}", e),
    }

    match validar_nome("", &nomes_existentes) {
        Ok(_) => println!("Nome válido"),
        Err(e) => println!("Erro de validação: {}", e),
    }

    match validar_nome("123 Danilo", &nomes_existentes) {
        Ok(_) => println!("Nome válido"),
        Err(e) => println!("Erro de validação: {}", e),
    }

    match validar_nome("Danilo", &nomes_existentes) {
        Ok(_) => println!("Nome válido"),
        Err(e) => println!("Erro de validação: {}", e),
    }

    match validar_nome("Alice", &nomes_existentes) {
        Ok(_) => println!("Nome válido"),
        Err(e) => {
            match e.downcast_ref::<ValidationError>().unwrap() {
                ValidationError::EmptyName(erro) => println!("Erro de validação: o nome não pode ser vazio - {}", erro),
                ValidationError::NonUniqueName(erro) => {
                    println!("{}", erro);
                    println!("Erro de validação: o nome não é único");
                    println!("Por favor, escolha um nome diferente.");
                },
                ValidationError::InvalidFormat(erro) => println!("Erro de validação: o formato do nome é inválido - {}", erro),
            }
        },
    }

    match validar_nome("Lia", &nomes_existentes) {
        Ok(_) => println!("Nome válido"),
        Err(e) => println!("Erro de validação: {}", e),
    }
}




/*

Use &dyn Trait quando você precisar de flexibilidade máxima 
e estiver disposto a aceitar o custo de desempenho do despacho dinâmico.

Use &dyn Trait para tipos mais complexos e quando for fazer polimorfismo

*/

/* 

Use &impl Trait para casos em que o desempenho é crítico 
e/ou quando você está trabalhando com funções que serão aplicadas a 
um único tipo concreto por chamada, aproveitando o despacho estático 
para eficiência.

Use &impl Trait para casos mais simples e leves onde não haverá polimorfismo:
*/

// trait Display {
//     fn display(&self) -> String;
// }

// struct Point {
//     x: i32,
//     y: i32,
// }

// impl Point {
//     fn mostrar(&self) -> String {
//         format!("Point(x: {}, y: {})", self.x, self.y)
//     }
// }

// impl Display for Point {
//     fn display(&self) -> String {
//         format!("Point(x: {}, y: {})", self.x, self.y)
//     }
// }

// struct Point2 {
//     x: i32,
//     y: i32,
// }

// impl Display for Point2 {
//     fn display(&self) -> String {
//         format!("Point(x: {}, y: {})", self.x, self.y)
//     }
// }

// fn print_display(item: &impl Display) {  // se estiver fazendo polimorfismo ou passando diversos tipos diferentes para minha trait, precisa mudar para &dyn
//     println!("{}", item.display());
// }

// fn main() {
//     let point = Point { x: 5, y: 10 };
//     point.mostrar();
//     point.display();

//     let point2 = Point2 { x: 5, y: 10 };

//     point2.display();

//     print_display(&point);
//     print_display(&point2);
// }