
fn retorno_da_funcao() -> i8{
    // return 1; // opção 1
    1  // opção 2
}  

fn escrever_algo(){ // função sem retorno
    println!("Oiiiiiiii");
}

fn funcao_com_parametros(valor: String, numero: i32) -> String{
    format!("{} - {}", valor, numero)
}

fn main() {
    let x = retorno_da_funcao();
    escrever_algo();
    escrever_algo();
    escrever_algo();

    println!("{}", x);

    let valor_concatenado = funcao_com_parametros("Danilo".to_string(), 40);
    println!("{}", valor_concatenado);
}