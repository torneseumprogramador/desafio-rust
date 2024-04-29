//// === Tokio para Programação Assíncrona ===

/*
Tokio é um runtime assíncrono para Rust, projetado para facilitar a escrita de 
código não bloqueante, especialmente útil para aplicações de I/O, 
como servidores web e clientes, bancos de dados e serviços de rede.
*/

// ///// ==== Exemplo blocante =======
/*
testar = curl localhost:8080 # Acessa e libera
testando = telnet localhost 8080 # bloqueia a requisição
testar = curl localhost:8080 # trava pois o telnet bloqueou
*/

// use std::net::TcpListener;
// use std::io::{self, Read, Write};

// fn main() -> io::Result<()> {
//     // Cria um ouvinte TCP que escuta na porta 8080 do localhost
//     let listener = TcpListener::bind("127.0.0.1:8080")?;
    
//     println!("Servidor rodando em 127.0.0.1:8080");

//     // Loop para aceitar conexões e lidar com elas
//     for stream in listener.incoming() {
//         match stream {
//             Ok(mut stream) => {
//                 println!("Cliente conectado");

//                 // Lê a mensagem do cliente
//                 let mut buf = [0; 1024];
//                 let bytes_read = stream.read(&mut buf)?;

//                 // Se a leitura foi bem-sucedida e bytes foram recebidos
//                 if bytes_read > 0 {
//                     let message = String::from_utf8_lossy(&buf[..bytes_read]);
//                     println!("Mensagem recebida: {}", message);

//                     // Envia uma resposta de volta para o cliente
//                     let response = "Mensagem recebida com sucesso!\n";
//                     stream.write_all(response.as_bytes())?;
//                 }

//                 // Fecha a conexão
//                 println!("Conexão encerrada");
//             }
//             Err(e) => {
//                 eprintln!("Erro ao aceitar conexão: {}", e);
//             }
//         }
//     }

//     Ok(())
// }



///// ==== Exemplo não blocante =======
/*
testar = curl localhost:8080 # Acessa e libera
testando = telnet localhost 8080 # bloqueia a requisição
testar = curl localhost:8080 # roda mesmo tendo outra requisição blocada vindo pelo telnet
*/

// use tokio::net::TcpListener;
// use tokio::io::{self, AsyncReadExt, AsyncWriteExt};

// #[tokio::main]
// async fn main() -> io::Result<()> {
//     // Cria um ouvinte TCP que escuta na porta 8080 do localhost
//     let listener = TcpListener::bind("127.0.0.1:8080").await?;

//     println!("Servidor rodando em 127.0.0.1:8080");

//     loop {
//         // Aceita uma conexão
//         let (mut socket, _) = listener.accept().await?;

//         println!("Cliente conectado");

//         // Usa `tokio::spawn` para lidar com a conexão em uma nova tarefa assíncrona
//         tokio::spawn(async move {
//             let mut buf = vec![0; 1024]; // Buffer para armazenar os dados recebidos

//             // Loop para ler dados do socket e escrever de volta (ecoar)
//             loop {
//                 match socket.read(&mut buf).await {
//                     // Se a leitura for bem-sucedida e alguns bytes forem recebidos
//                     Ok(n) if n == 0 => {
//                         println!("Conexão encerrada pelo cliente");
//                         return;
//                     },
//                     Ok(n) => {
//                         // Escreve os dados de volta para o socket (ecoar)
//                         if socket.write_all(&buf[..n]).await.is_err() {
//                             println!("Erro ao enviar dados de volta");
//                             return;
//                         }
//                     },
//                     Err(e) => {
//                         println!("Erro ao ler do socket: {:?}", e);
//                         return;
//                     }
//                 }
//             }
//         });
//     }
// }






// //// ==== programação Assincrona - async_std ========
/*
Programação Assíncrona
Modelo de Execução: Na programação assíncrona, uma única thread pode executar múltiplas tarefas quase simultaneamente.

Threads
Modelo de Execução: Threads permitem a execução de múltiplas tarefas em paralelo, literalmente ao mesmo tempo, em sistemas com múltiplos cores de processador
*/

// use async_std::task;

// async fn say_hello(item: String) -> String {
//     println!("Hello, async world! - {}", item);
//     format!("{} - ret - ", item)
// }

// fn main() {
//     let item = task::block_on(say_hello("01".to_string())); // o await
//     task::block_on(say_hello(item));

//     //say_hello("03"); não roda pois não em o await
// }


// // ==== programação Assincrona ========
// use async_std::task;

// async fn say_hello(item: String) -> String {
//     println!("Hello, async world! - {}", item);
//     format!("{} - ret - ", item)
// }

// async fn run() -> String {
//     let item = say_hello("01".to_string()).await;
//     let item2 = say_hello(item).await;
//     item2
// }

// fn main() {
//     let item = task::block_on(run());
//     println!("Item - {}", item);
// }



//// ==== programação Assincrona smol runtime ========
// O smol é um runtime assíncrono pequeno e eficiente para Rust, projetado para ser leve e fácil de usar. 

// fn main() {
//     smol::block_on(async {
//         println!("Hello, smol world!");
//     });
// }


// //// ==== programação Assincrona smol + surf runtime ========
// async fn fetch_url(url: &str) -> surf::Result<String> {
//     let mut res = surf::get(url).await?;
//     println!("Status: {} - {}", res.status(), url);
//     // Aqui precisamos aguardar o resultado do body_string, pois é uma operação assíncrona.
//     let body = res.body_string().await?;
//     Ok(body) // Retornamos o corpo da resposta como uma String.
// }

// fn main() {
//     // Agora vamos também tratar o caso de sucesso do fetch_url, não apenas o erro.
//     match smol::block_on(fetch_url("https://google.com")) {
//         Ok(body) => println!("Corpo da resposta do Google.com: {}", body),
//         Err(e) => eprintln!("Erro ao buscar https://google.com: {}", e),
//     }

//     match smol::block_on(fetch_url("https://httpbin.org/get")) {
//         Ok(body) => println!("Corpo da resposta do httpbin.org/get: {}", body),
//         Err(e) => eprintln!("Erro ao buscar https://httpbin.org/get: {}", e),
//     }
// }



// //// ==== programação Assincrona read_file ========

// use async_std::fs::File;
// use async_std::prelude::*;
// use async_std::task;

// async fn read_file() -> std::io::Result<()> {
//     let mut file = File::open("danilo.txt").await?;
//     let mut contents = String::new();
//     file.read_to_string(&mut contents).await?;
//     println!("File contents: {}", contents);
//     Ok(())
// }

// fn main() {

//     match task::block_on(read_file()) {
//         Ok(_) => println!("Arquivo lido"),
//         Err(e) => eprintln!("erro ao ler arquivo: {}", e),
//     }
    
// }



// //// ==== programação Assincrona ========
// use async_std::task;
// use std::time::Duration;

// async fn task_one() -> i32 {
//     task::sleep(Duration::from_millis(500)).await;
//     println!("task_one");
//     10
// }

// async fn task_two() -> i32 {
//     task::sleep(Duration::from_millis(200)).await;
//     println!("task_two");
//     20
// }

// fn main() {
//     smol::block_on(async {
//         let t1 = smol::spawn(task_one());
//         let t2 = smol::spawn(task_two());

//         let result_one = t1.await;
//         let result_two = t2.await;

//         println!("Results: {}, {}", result_one, result_two);
//     });
// }



//// ==== programação Assincrona ========
use async_std::task;
use futures::join;
use std::time::Duration;


async fn task_one() -> i32 {
    task::sleep(Duration::from_millis(500)).await;
    println!("task_one");
    10
}

async fn task_two() -> i32 {
    task::sleep(Duration::from_millis(200)).await;
    println!("task_two");
    20
}

fn main() {
    task::block_on(async {
        // Usando `join!` diretamente conforme sugerido pela mensagem de erro do compilador
        let (result_one, result_two) = join!(task_one(), task_two());
        println!("Results: {}, {}", result_one, result_two);
    });
}