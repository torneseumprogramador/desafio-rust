// trait Display {
//     fn display(&self) -> String;
// }

// struct Point {
//     x: i32,
//     y: i32,
// }

// impl Display for Point {
//     fn display(&self) -> String {
//         format!("Point(x: {}, y: {})", self.x, self.y)
//     }
// }

// struct Circle {
//     x: i32,
//     y: i32
// }

// impl Display for Circle {
//     fn display(&self) -> String {
//         format!("Circle(x: {}, y: {})", self.x, self.y)
//     }
// }


// #[derive(Debug)]
// enum Type {
//     Circle,
//     Point
// }

// impl Display for Type {
//     fn display(&self) -> String {
//         format!("Type: {:?}", self)
//     }
// }

// // fn print_display(item: &dyn Display) {
// fn print_display(item: &impl Display) {
//     println!("{}", item.display());
// }

// fn main() {
//     let point = Point { x: 5, y: 10 };
//     let circle = Circle { x: 15, y: 25 };
//     let type_circle = Type::Circle;
//     let type_point = Type::Point;

//     print_display(&point);
//     print_display(&circle);
//     print_display(&type_circle);
//     print_display(&type_point);
// }



// // ========= Polimorfismo ============
// trait Display {
//     fn display(&self) -> String;
// }

// struct Point {
//     x: i32,
//     y: i32,
// }

// impl Display for Point {
//     fn display(&self) -> String {
//         format!("Point(x: {}, y: {})", self.x, self.y)
//     }
// }

// struct Circle {
//     x: i32,
//     y: i32,
//     radius: i32,
// }

// impl Display for Circle {
//     fn display(&self) -> String {
//         format!("Circle(x: {}, y: {}, radius: {})", self.x, self.y, self.radius)
//     }
// }

// fn print_display(item: &impl Display) {
//     println!("{}", item.display());
// }

// fn main() {
//     let point = Point { x: 5, y: 10 };
//     let circle = Circle { x: 15, y: 25, radius: 5 };

//     print_display(&point);
//     print_display(&circle);
// }



// use std::fs::OpenOptions;
// use std::io::prelude::*;

// trait Logger {
//     fn log(&self, message: &str);
// }

// struct ConsoleLogger;

// impl Logger for ConsoleLogger {
//     fn log(&self, message: &str) {
//         println!("Console Logger: {}", message);
//     }
// }

// struct FileLogger;

// impl Logger for FileLogger {
//     fn log(&self, message: &str) {
//         if let Ok(mut file) = OpenOptions::new().create(true).append(true).open("info.log") {
//             if let Err(e) = writeln!(file, "{}", message) {
//                 eprintln!("Erro ao escrever no arquivo: {}", e);
//             }
//         } else {
//             eprintln!("Erro ao abrir o arquivo info.log");
//         }
//     }
// }

// fn log_message(logger: &impl Logger, message: &str) {
//     logger.log(message);
// }

// fn main() {
//     let console_logger = ConsoleLogger;
//     let file_logger = FileLogger;

//     log_message(&console_logger, "Hello, world!");
//     log_message(&file_logger, "Logging to a file now.");
// }


// trait Display {
//     fn display(&self) -> String;
// }

// struct Point {
//     x: i32,
//     y: i32,
// }

// impl Display for Point {
//     fn display(&self) -> String {
//         format!("Point(x: {}, y: {})", self.x, self.y)
//     }
// }

// struct Circle {
//     x: i32,
//     y: i32,
//     radius: i32,
// }

// impl Display for Circle {
//     fn display(&self) -> String {
//         format!("Circle(x: {}, y: {}, radius: {})", self.x, self.y, self.radius)
//     }
// }


// /* 

// Use &impl Trait para casos em que o desempenho é crítico 
// e/ou quando você está trabalhando com funções que serão aplicadas a 
// um único tipo concreto por chamada, aproveitando o despacho estático 
// para eficiência.

// Use &impl Trait para casos mais simples e leves onde não haverá polimorfismo:
// */

// fn print_display_single_with_impl(item: &impl Display) { // &impl Display(Trait) = "Sintaxe de Traits Concretos" (Syntactic Sugar for Concrete Trait Bounds)
//     println!("{}", item.display());
// }

// /*
// &dyn Display usa o que é conhecido como "Trait Objects". 
// Isso é mais apropriado para casos em que você precisa armazenar 
// múltiplos tipos que implementam o mesmo trait em uma única coleção 
// ou quando você precisa de flexibilidade para lidar com vários tipos 
// em tempo de execução.

// Use &dyn Trait quando você precisar de flexibilidade máxima 
// e estiver disposto a aceitar o custo de desempenho do despacho dinâmico.

// Use &dyn Trait para tipos mais complexos e quando for fazer polimorfismo

// */

// // Usando &dyn Display ( se estivesse montando um Pattern Observer por exemplo )
// fn print_display_multiple_with_dyn(items: &[&dyn Display]) { // &dyn Display(Trait) = "Trait Objects"
//     for item in items {
//         println!("{}", item.display());
//     }
// }

// fn main() {
//     let point = Point { x: 5, y: 10 };
//     let circle = Circle { x: 15, y: 25, radius: 5 };

//      // Chama com &impl Display
//      print_display_single_with_impl(&point);
//      print_display_single_with_impl(&circle);
 
//      // Chama com &dyn Display
//      let shapes: [&dyn Display; 2] = [&point, &circle];
//      print_display_multiple_with_dyn(&shapes);
// }



///// ===== Observer Design Pattern =======

trait Observer {
    fn send(&self, message: &String, cliente: &Cliente);
}

struct NotificationSender<'a> {
    observers: Vec<&'a dyn Observer>,
}

impl<'a> NotificationSender<'a> {
    // Método para adicionar observadores
    fn add_observer(&mut self, observer: &'a dyn Observer) {
        self.observers.push(observer);
    }

    // Método para remover observadores
    fn remove_observer(&mut self, observer: &'a dyn Observer) {
        self.observers.retain(|&o| o as *const _ != observer as *const _);
    }
    // Método para notificar todos os observadores
    fn notify_observers(&self, message: &String, cliente: &Cliente) {
        for observer in &self.observers {
            observer.send(message, cliente);
        }
    }

    // Método para enviar e-mails em lote
    fn send_batch(&self, clientes: &Vec<Cliente>) {
        // Lógica para enviar e-mails em lote
        // Aqui você pode adicionar sua lógica de envio de e-mails
        
        // Após o envio, notifique os observadores
        for cliente in clientes.iter() {
            let mensagem = format!("Oi {}, estamos com uma promoção idela para você, entre em contato no whatsapp 11 97773-2292", cliente.nome);
            self.notify_observers(&mensagem, cliente);
            println!("-----------");
        }
    }
}


struct EmailSender;
impl Observer for EmailSender {
    fn send(&self, message: &String, cliente: &Cliente){
        println!("TODO - Enviando Email para: {}, com a mensagem - {}", cliente.email, message);
    }
}


struct SnsSender;
impl Observer for SnsSender {
    fn send(&self, message: &String, cliente: &Cliente){
        println!("TODO - Enviando SMS para: {}, com a mensagem - {}", cliente.telefone, message);
    }
}

struct WhatsappSender;
impl Observer for WhatsappSender {
    fn send(&self, message: &String, cliente: &Cliente){
        println!("TODO - Enviando Whatsapp para: {}, com a mensagem - {}", cliente.telefone, message);
    }
}

struct Cliente {
    nome:String,
    email:String,
    telefone:String
}

fn main(){

    let mut sender_batch = NotificationSender{ observers: Vec::new() };
        
    // Adicionando observadores

    let email_sender = EmailSender{};
    let sms_sender = SnsSender{};
    let whatsap_sender_sender = WhatsappSender{};

    sender_batch.add_observer(&email_sender);
    sender_batch.add_observer(&sms_sender);
    sender_batch.add_observer(&whatsap_sender_sender);

    // sender_batch.remove_observer(&sms_sender);
    
    // Enviando e-mails em lote
    let mut emails: Vec<Cliente> = Vec::new();
    emails.push(Cliente { nome: "Herick".to_string(), email: "email1@example.com".to_string(), telefone: "11 1111-1111".to_string() });
    emails.push(Cliente { nome: "Eduardo".to_string(), email: "email2@example.com".to_string(), telefone: "11 1111-1311".to_string() });
    emails.push(Cliente { nome: "Luciano".to_string(), email: "email3@example.com".to_string(), telefone: "11 1111-1411".to_string() });

    sender_batch.send_batch(&emails);
}