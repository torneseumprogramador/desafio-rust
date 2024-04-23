use redis::{Commands, RedisError, RedisResult};
use serde_json;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Pessoa {
    id: i32,
    nome: String,
    curriculo: String,
}

fn save_pessoa(con: &mut redis::Connection, pessoa: &Pessoa) -> RedisResult<()> {
    let serialized = serde_json::to_string(pessoa)
        .map_err(|e| RedisError::from((redis::ErrorKind::TypeError, "Serialization error", e.to_string())))?;
    
    let key = format!("pessoa:{}", pessoa.id);
    con.set(key, serialized)?;
    Ok(())
}

fn get_pessoa(con: &mut redis::Connection, id: i32) -> RedisResult<Pessoa> {
    let key = format!("pessoa:{}", id);

    let data: String = con.get(key)
        .map_err(|e| RedisError::from((redis::ErrorKind::TypeError, "Deserialization error", e.to_string())))?;
    let pessoa: Pessoa = serde_json::from_str(&data)
        .map_err(|e| RedisError::from((redis::ErrorKind::TypeError, "Deserialization error", e.to_string())))?;
    Ok(pessoa)
}

fn main() -> RedisResult<()> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;

    let pessoa = Pessoa {
        id: 1,
        nome: String::from("João Silva"),
        curriculo: String::from("Desenvolvedor de Software"),
    };

    // Salvar a Pessoa
    save_pessoa(&mut con, &pessoa)?;

    // Recuperar a Pessoa
    let retrieved_pessoa = get_pessoa(&mut con, 1)?;
    println!("Pessoa recuperada: {:?}", retrieved_pessoa);






    // Criando uma lista de pessoas
    let pessoas = vec![
        Pessoa {
            id: 1,
            nome: String::from("João Silva"),
            curriculo: String::from("Desenvolvedor de Software"),
        },
        Pessoa {
            id: 2,
            nome: String::from("Maria Fernanda"),
            curriculo: String::from("Cientista de Dados"),
        },
    ];

    // Serializa a lista de pessoas em uma string JSON
    let serialized_pessoas = serde_json::to_string(&pessoas)
        .expect("Falha ao serializar pessoas");

    // Armazenando a string JSON no Redis
    con.set("lista_pessoas", serialized_pessoas)?;
    
    // Definindo o tempo de expiração para 30 segundos
    con.expire("lista_pessoas", 30)?;


    // Recuperando a string JSON da lista de pessoas
    let pessoas_data: String = con.get("lista_pessoas")
        .expect("Falha ao recuperar a lista de pessoas");

    // Desserializa a string JSON de volta para a lista de structs Pessoa
    let retrieved_pessoas: Vec<Pessoa> = serde_json::from_str(&pessoas_data)
        .expect("Falha ao desserializar pessoas");

    println!("Pessoas recuperadas: {:?}", retrieved_pessoas);


    Ok(())
}
