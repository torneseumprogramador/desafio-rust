// ///// ========== Usando Cria struct em arquivo separado =========
// #[macro_use]
// extern crate minha_macro_lib;

// cria_struct! {
//     Cliente {
//         id: u32,
//         nome: String,
//         cpf: String,
//     }

//     // Definição de métodos
//     fn mostra_nome(&self) -> String {
//         format!("Nome: {}", self.nome)
//     }

//     fn mostra_id(&self) -> String {
//         format!("ID: {}", self.id)
//     }

//     fn mostra_cpf(&self) -> String {
//         format!("CPF: {}", self.cpf)
//     }

//     fn mais_numero_no_id(&self, numero: u32) -> String {
//         format!("ID + Numero: {}", self.id + numero)
//     }
// }

// fn main() {
//     // Agora podemos usar a struct Cliente após sua definição
//     let cliente = Cliente {
//         id: 1,
//         nome: "João da Silva".to_string(),
//         cpf: "123.456.789-00".to_string(),
//     };

//     println!("{}", cliente.mostra_nome());
//     println!("{}", cliente.mostra_id());
//     println!("{}", cliente.mostra_cpf());
//     println!("{}", cliente.mais_numero_no_id( 10 ));
// }






// ///// ========== Usando Cria struct em arquivo separado =========
// extern crate minha_macro_lib;

// use minha_macro_lib::create_struct_and_metadata;

// // Usando a macro para criar a struct Cliente com metadados
// create_struct_and_metadata! {
//     Cliente {
//         id: i32, "autoincrement",
//         nome: String, "varchar(100)",
//         telefone: String, "varchar(15)",
//     }
// }

// fn main() {
//     for (field, field_type, metadata) in Cliente::metadata() {
//         println!("Field: {}, Type: {}, Metadata: {}", field, field_type, metadata);
//     }
// }



// // Definição da struct Cliente usando a macro atualizada

// extern crate minha_macro_lib;
// use minha_macro_lib::create_struct_and_metadata_com_metodo;

// create_struct_and_metadata_com_metodo! {
//     Cliente {
//         id: i32, "autoincrement",
//         nome: String, "varchar(100)",
//         telefone: String, "varchar(15)",
//     }
// }

// create_struct_and_metadata_com_metodo! {
//     Pedido {
//         id: i32, "autoincrement",
//         cliente_id: i32, "int",
//         valor_total: f32, "float",
//     }
// }

// fn main() {
//     // Gerando o script SQL para criar a tabela `cliente`
//     let sql_create_table = Cliente::generate_sql_create_table();
//     println!("{}", sql_create_table);


//     let sql_create_table_pedito = Pedido::generate_sql_create_table();
//     println!("{}", sql_create_table_pedito);
// }




// // Definindo a struct Cliente com nome de tabela personalizado usando a macro atualizada
// extern crate minha_macro_lib;
// use minha_macro_lib::create_struct_and_metadata_com_metodo_v2;

// create_struct_and_metadata_com_metodo_v2! {
//     "tbl_clientes" => Cliente {
//         id: i32, "autoincrement",
//         nome: String, "varchar(100)",
//         telefone: String, "varchar(15)",
//     }
// }


// fn main() {
//     // Gerando e exibindo o script SQL para criar a tabela `clientes`
//     let sql_create_table = Cliente::generate_sql_create_table();
//     println!("{}", sql_create_table);
// }






// Definindo a struct Cliente com metodos que geram o SQL
extern crate minha_macro_lib;
use minha_macro_lib::create_struct_and_metadata_com_sql_methods;

create_struct_and_metadata_com_sql_methods! {
    "clientes" => Cliente {
        id: i32, "autoincrement",
        nome: String, "varchar(100)",
        telefone: String, "varchar(15)",
    }
}



fn main() {
    println!("SQL Create Table:\n{}\n", Cliente::generate_sql_create_table());
    println!("SQL Drop Table:\n{}\n", Cliente::generate_sql_drop_table());
    println!("SQL Insert:\n{}\n", Cliente::generate_sql_insert());
    println!("SQL Update:\n{}\n", Cliente::generate_sql_update());
    println!("SQL Delete:\n{}\n", Cliente::generate_sql_delete());
    println!("SQL Select:\n{}\n", Cliente::generate_sql_select());
}