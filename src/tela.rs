use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

pub fn limpar_tela() {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
                .args(&["/C", "cls"])
                .status()
                .unwrap();
    } else {
        Command::new("clear")
                .status()
                .unwrap();
    }
}

pub fn esperar(tempo:u64){
    sleep(Duration::from_secs(tempo)); 
}

pub fn mostrar_mensagem(mensagem: &str){
    limpar_tela();
    println!("{}", mensagem);
    esperar(2);
}