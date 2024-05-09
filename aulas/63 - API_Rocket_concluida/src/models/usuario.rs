use crate::orm_desafio_v1::create_struct_and_metadata_com_sql_methods;

create_struct_and_metadata_com_sql_methods! {
    "usuarios" => Usuario {
        id: i32, "autoincrement",
        nome: String, "varchar(150)",
        email: String, "varchar(255)",
        senha: String, "varchar(100)"
    }
}