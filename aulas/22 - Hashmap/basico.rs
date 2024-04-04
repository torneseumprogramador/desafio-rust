use std::collections::HashMap;

fn main(){
    let mut config_db: HashMap<&str, &str> = HashMap::new();

    config_db.insert("string_postgres", "Driver={PostgreSQL};Server=IP address;Port=5432;Database=myDataBase;Uid=myUsername;Pwd=myPassword;");
    config_db.insert("string_mysql", "Provider=MySQLProv;Data Source=mydb;User Id=myUsername;Password=myPassword;");

    for (key, value) in &config_db {
        println!("{}: {}", key, value);
    }
}

