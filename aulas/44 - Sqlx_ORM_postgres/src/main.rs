use sqlx::postgres::PgPool;
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok(); // Carrega as variáveis de ambiente do arquivo .env
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Conecta ao banco de dados usando PgPool diretamente
    let pool = PgPool::connect(&database_url).await?;

    // Inserir dados
    let mut tx = pool.begin().await?;
    sqlx::query("INSERT INTO students (name, registration, grades) VALUES ($1, $2, $3)")
        .bind("Alice")
        .bind("123456")
        .bind("A+")
        .execute(&mut *tx)
        .await?;
    tx.commit().await?;

    // Consultar dados
    let rows = sqlx::query_as!(
        Student,
        "SELECT id, name, registration, grades FROM students"
    )
    .fetch_all(&pool)
    .await?;

    for row in rows {
        println!("ID: {}, Name: {}, Registration: {}, Grades: {}",
                 row.id, 
                 row.name.unwrap_or_default(), 
                 row.registration.unwrap_or_default(), 
                 row.grades.unwrap_or_default());
    }

    Ok(())
}

#[derive(sqlx::FromRow)]
struct Student {
    id: i32,
    name: Option<String>,
    registration: Option<String>,
    grades: Option<String>,
}
