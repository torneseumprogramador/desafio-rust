use crate::orm_desafio_v1::create_struct_and_metadata_com_sql_methods;

create_struct_and_metadata_com_sql_methods! {
    "alunos" => Aluno {
        id: i32, "autoincrement",
        nome: String, "varchar(150)",
        matricula: String, "varchar(50)",
    }
}