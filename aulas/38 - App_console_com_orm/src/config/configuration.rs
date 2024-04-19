use std::env;

pub fn get_mysql_string_connection() -> String {
    let user:String = match env::var("DATABASE_USER") {
        Ok(value) => value,
        Err(_) => "root".to_string(),
    };

    let pass:String = match env::var("DATABASE_PASSWORD") {
        Ok(value) => value,
        Err(_) => "".to_string(),
    };

    let db:String = match env::var("DATABASE_DB") {
        Ok(value) => value,
        Err(_) => "seu_db".to_string(),
    };

    let host:String = match env::var("DATABASE_HOST") {
        Ok(value) => value,
        Err(_) => "localhost".to_string(),
    };

    format!("mysql://{}:{}@{}/{}", user, pass, host, db)
}