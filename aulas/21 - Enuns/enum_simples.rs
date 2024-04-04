use std::fmt::Display;

#[derive(Debug)]
enum VersaoIp {
    V4,
    V6,
}

struct Servidor {
    nome: String,
    tipo_ip: VersaoIp
}

fn mostra_dados_servidores(servidor: Servidor){
    println!("Nome: {}", servidor.nome);
    println!("Tipo: {:?}", servidor.tipo_ip);
}

fn main(){
    let servidor1 = Servidor{
        nome: "Servidor Rust deploy 01".to_string(),
        tipo_ip: VersaoIp::V4
    };

    let servidor2 = Servidor{
        nome: "Servidor Rust deploy 02".to_string(),
        tipo_ip: VersaoIp::V6
    };

    mostra_dados_servidores(servidor1);
    mostra_dados_servidores(servidor2);
}