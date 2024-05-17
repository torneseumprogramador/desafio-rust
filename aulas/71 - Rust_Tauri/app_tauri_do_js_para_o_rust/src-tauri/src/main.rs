#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
struct Cliente {
    id: i32,
    nome: String,
    endereco: String,
}

#[tauri::command]
fn funcao_com_retorno() -> Vec<Cliente> {
    println!("Lista de clientes");
    
    let clientes = vec![
      Cliente { id: 1, nome: String::from("Reinaldo"), endereco: String::from("Rua teste") },
      Cliente { id: 2, nome: String::from("Luan"), endereco: String::from("Rua das ameixeiras") }
    ];

    clientes
}

#[tauri::command]
fn adiciona_na_lista(cliente: Cliente) -> Vec<Cliente> {
  println!("aqui ===");
  
  vec![
    cliente
  ]
}

#[tauri::command]
fn escreve_nome_no_rust(nome: String) {
  println!("O nome chamado foi {}", nome);
}

fn main() {
  tauri::Builder::default()
      .invoke_handler(tauri::generate_handler![
        funcao_com_retorno,
        escreve_nome_no_rust,
        adiciona_na_lista
      ])
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
}