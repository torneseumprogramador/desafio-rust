struct Conexao {
    driver: String,
    str_conexao: String
}

fn main(){
    let conexao = Conexao {
        driver: String::from("PostgreSQL"),
        str_conexao: "Server=IP address;Port=5432;Database=myDataBase;Uid=myUsername;Pwd=myPassword;".to_string()
    };

    println!("Driver: {}, Str: {}", conexao.driver, conexao.str_conexao);
}

