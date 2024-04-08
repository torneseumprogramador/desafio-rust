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
    mostrar_mensagem_controlando_tempo(mensagem, 2)
}

pub fn mostrar_mensagem_controlando_tempo(mensagem: &str, tempo: u64){
    limpar_tela();
    println!("{}", mensagem);
    esperar(tempo);
}