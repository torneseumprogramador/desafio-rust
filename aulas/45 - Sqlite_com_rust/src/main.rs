use rusqlite::{params, Connection, Result, Row};

#[derive(Debug)]
struct Pessoa {
    id: i32,
    nome: String,
    curriculo: String,
}

fn main() -> Result<()> {
    let conn = Connection::open("db/meu_banco.db")?;

    // Cria a tabela se ela não existir
    conn.execute(
        "CREATE TABLE IF NOT EXISTS pessoas (id INTEGER PRIMARY KEY, nome VARCHAR(100) NOT NULL, curriculo TEXT NOT NULL)",
        [],
    )?;

    // Insere uma nova pessoa
    let pessoa = Pessoa {
        id: 0,
        nome: String::from("Valter"),
        curriculo: String::from("Um programador que entende de infra e devops e agora de rust"),
    };

    conn.execute(
        "INSERT INTO pessoas (nome, curriculo) VALUES (?1, ?2)",
        params![pessoa.nome, pessoa.curriculo],
    )?;

    // Atualiza dados de uma pessoa
    update_pessoa(&conn, 1, "Novo nome", "Novo currículo")?;

    // Deleta uma pessoa
    delete_pessoa(&conn, 1)?;

    // Busca por uma pessoa pelo ID
    if let Ok(alguma_pessoa) = find_pessoa_by_id(&conn, 2) {
        println!("Pessoa encontrada: {:?}", alguma_pessoa);
    }

    // Lista todas as pessoas
    let mut stmt = conn.prepare("SELECT * FROM pessoas")?;
    let pessoa_iter = stmt.query_map([], |row| {
        let id: i32 = row.get(0)?;
        let nome: String = row.get(1)?;
        let curriculo: String = row.get(2)?;
        Ok(Pessoa { id, nome, curriculo })
    })?;

    for pessoa in pessoa_iter {
        println!("Encontrado pessoa: {:?}", pessoa?);
    }

    Ok(())
}

fn update_pessoa(conn: &Connection, id: i32, novo_nome: &str, novo_curriculo: &str) -> Result<()> {
    conn.execute(
        "UPDATE pessoas SET nome = ?1, curriculo = ?2 WHERE id = ?3",
        params![novo_nome, novo_curriculo, id],
    )?;
    Ok(())
}

fn delete_pessoa(conn: &Connection, id: i32) -> Result<()> {
    conn.execute(
        "DELETE FROM pessoas WHERE id = ?1",
        params![id],
    )?;
    Ok(())
}

fn find_pessoa_by_id(conn: &Connection, id: i32) -> Result<Pessoa> {
    conn.query_row(
        "SELECT id, nome, curriculo FROM pessoas WHERE id = ?1",
        params![id],
        |row| {
            Ok(Pessoa {
                id: row.get(0)?,
                nome: row.get(1)?,
                curriculo: row.get(2)?,
            })
        }
    )
}
