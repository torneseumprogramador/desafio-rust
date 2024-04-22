extern crate diesel;

mod models;
mod schema;

use crate::diesel::Connection;
use crate::diesel::RunQueryDsl;
use crate::diesel::ExpressionMethods;
use crate::diesel::QueryDsl;
use models::student::Student;
use diesel::PgConnection;
use schema::students;

fn main() -> Result<(), diesel::result::Error> {
    let mut connection = establish_connection();

    // Inserir dados
    let new_student = (
        students::name.eq("Alice"),
        students::registration.eq("123456"),
        students::grades.eq("A+"),
    );

    let last_id: i32 = diesel::insert_into(students::table) // Ajuste o tipo de `inserted_id` conforme o tipo do seu ID
        .values(&new_student)
        .returning(students::id) // Aqui você especifica que quer retornar o `id`
        .get_result(&mut connection)?;

    // Atulizar dados
    diesel::update(schema::students::table.filter(schema::students::id.eq(last_id)))
        .set((schema::students::name.eq("Nome Atualizado"),
              schema::students::grades.eq("A-")))
        .execute(&mut connection)?;


    // Excluir dados
    diesel::delete(schema::students::table.filter(schema::students::id.eq(last_id)))
        .execute(&mut connection)?;
    


    // Inserir dados
    let last_id_para_buscar: i32 = diesel::insert_into(schema::students::table)
        .values((
            schema::students::name.eq("Dado para buscar"),
            schema::students::registration.eq("123456"),
            schema::students::grades.eq("A+")
        ))
        .returning(students::id)
        .get_result(&mut connection)?;

    // Buscar por id
    let result = schema::students::table
        .filter(schema::students::id.eq(last_id_para_buscar))
        .first::<Student>(&mut connection)?;

    println!("{:?}", result);
    

    // Buscar por registration
    let result = schema::students::table
        .filter(schema::students::registration.eq("123456"))
        .load::<Student>(&mut connection)?;

    for student in result {
        println!("{:?}", student);
    }


    // Buscar paginado
    let results = schema::students::table
        .offset(0) // deixa o indice a partir de 0
        .limit(5)   // Retorna os próximos 5 registros
        .load::<Student>(&mut connection)?;

    for student in results {
        println!("{:?}", student);
    }


    // Buscar dados
    let results = schema::students::table.load::<Student>(&mut connection)?;

    for student in results {
        println!("{}: {}", student.name, student.grades.unwrap() );
    }

    Ok(())
}

fn establish_connection() -> PgConnection {
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
