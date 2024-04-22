use sqlx::MySql;
use sqlx::Pool;
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = Pool::<MySql>::connect(&database_url).await?;
    
    // Inserir dados
    let mut tx = pool.begin().await?;
    sqlx::query("INSERT INTO students (name, registration, grades) VALUES (?, ?, ?)")
        .bind("Alice")
        .bind("123456")
        .bind("A+")
        .execute(&mut *tx)
        .await?;
    tx.commit().await?;

    // Consultar dados
    let rows = sqlx::query!("SELECT id, name, registration, grades FROM students")
        .fetch_all(&pool)
        .await?;

    for row in rows {
        println!("ID: {}, Name: {}, Registration: {}, Grades: {}", row.id, row.name.unwrap(), row.registration.unwrap(), row.grades.unwrap());
    }

    Ok(())
}
