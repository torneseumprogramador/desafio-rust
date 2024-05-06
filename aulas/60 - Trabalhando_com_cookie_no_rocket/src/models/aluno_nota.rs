use crate::orm_desafio_v1::create_struct_and_metadata_com_sql_methods;

create_struct_and_metadata_com_sql_methods! {
    "alunos_notas" => AlunoNota {
        id: i32, "autoincrement",
        aluno_id: i32, "int",
        nota: f32, "float",
    }
}