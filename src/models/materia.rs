use orm_desafio_v1::create_struct_and_metadata_com_sql_methods;

create_struct_and_metadata_com_sql_methods! {
    "materias" => Materia {
        id: i32, "autoincrement",
        titulo: String, "varchar(150)",
        descricao: String, "text",
    }
}