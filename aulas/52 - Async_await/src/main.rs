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






//// ==== programação Assincrona - async_std ========
/*
Programação Assíncrona
Modelo de Execução: Na programação assíncrona, uma única thread pode executar múltiplas tarefas quase simultaneamente.

Threads
Modelo de Execução: Threads permitem a execução de múltiplas tarefas em paralelo, literalmente ao mesmo tempo, em sistemas com múltiplos cores de processador
*/

use async_std::task;

async fn say_hello(item: String) -> String {
    println!("Hello, async world! - {}", item);
    format!("{} - ret - ", item)
}

fn main() {
    let item = task::block_on(say_hello("01".to_string())); // o await
    task::block_on(say_hello(item));

    //say_hello("03"); não roda pois não em o await
}