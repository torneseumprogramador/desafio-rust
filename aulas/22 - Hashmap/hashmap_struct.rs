use std::collections::HashMap;

struct Conexao {
    driver: String,
    str_conexao: String
}

fn main(){
    let mut config_db: HashMap<&str, Conexao> = HashMap::new();

    config_db.insert("string_postgres", Conexao {
        driver: String::from("PostgreSQL"),
        str_conexao: "Server=IP address;Port=5432;Database=myDataBase;Uid=myUsername;Pwd=myPassword;".to_string()
    });

    config_db.insert("string_mysql", Conexao {
        driver: String::from("MySQLProv"),
        str_conexao: "Data Source=mydb;User Id=myUsername;Password=myPassword;".to_string()
    });

    for (key, value) in &config_db {
        println!("{}: Driver: {}, Str: {}", key, value.driver, value.str_conexao);
    }
}

