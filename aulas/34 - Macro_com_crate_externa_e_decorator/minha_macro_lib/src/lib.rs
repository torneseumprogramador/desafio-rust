#[macro_export]
macro_rules! cria_struct {
    (
        $nome_struct:ident {
            $($campo:ident: $tipo:ty),* $(,)?
        }
        $(fn $nome_metodo:ident(&$nome_metodo_struct:ident $(, $param_nome:ident: $param_tipo:ty)*) -> $ret_tipo:ty $corpo_metodo:block)*
    ) => {
        struct $nome_struct {
            $($campo: $tipo,)*
        }

        impl $nome_struct {
            $(
                fn $nome_metodo(&$nome_metodo_struct $(, $param_nome: $param_tipo)*) -> $ret_tipo $corpo_metodo
            )*
        }
    };
}


#[macro_export]
macro_rules! create_struct_and_metadata {
    ($name:ident { $($field_name:ident: $field_type:ty, $metadata:expr),* $(,)? }) => {
        // Definição da struct
        struct $name {
            $(pub $field_name: $field_type,)*
        }
        
        // Armazenando os metadados em uma função associada
        impl $name {
            pub fn metadata() -> Vec<(&'static str, &'static str, &'static str)> {
                vec![
                    $( (stringify!($field_name), stringify!($field_type), $metadata), )*
                ]
            }
        }
    };
}

#[macro_export]
macro_rules! create_struct_and_metadata_com_metodo {
    ($name:ident { $($field_name:ident: $field_type:ty, $metadata:expr),* $(,)? }) => {
        // Definição da struct
        struct $name {
            $(pub $field_name: $field_type,)*
        }
        
        // Armazenando os metadados e gerando SQL
        impl $name {
            pub fn metadata() -> Vec<(&'static str, &'static str, &'static str)> {
                vec![
                    $( (stringify!($field_name), stringify!($field_type), $metadata), )*
                ]
            }

            // Nova função para gerar a query SQL de criação da tabela
            pub fn generate_sql_create_table() -> String {
                let table_name = stringify!($name).to_lowercase();
                let columns = Self::metadata().iter().map(|(field, _type, meta)| {
                    let sql_type = match *meta {
                        "autoincrement" => "INT AUTO_INCREMENT PRIMARY KEY",
                        _ => meta,
                    };
                    format!("{} {}", field, sql_type)
                }).collect::<Vec<String>>().join(",\n    ");
                
                format!("CREATE TABLE {} (\n    {}\n);", table_name, columns)
            }
        }
    };
}


///// ==== Exemplo 3 =============
#[macro_export]
macro_rules! create_struct_and_metadata_com_metodo_v2 {
    ($table_name:expr => $struct_name:ident { $($field_name:ident: $field_type:ty, $metadata:expr),* $(,)? }) => {
        // Definição da struct
        struct $struct_name {
            $(pub $field_name: $field_type,)*
        }
        
        // Armazenando os metadados e gerando SQL com nome de tabela personalizado
        impl $struct_name {
            pub fn metadata() -> Vec<(&'static str, &'static str, &'static str)> {
                vec![
                    $( (stringify!($field_name), stringify!($field_type), $metadata), )*
                ]
            }

            // Função atualizada para gerar a query SQL de criação da tabela com nome personalizado
            pub fn generate_sql_create_table() -> String {
                let columns = Self::metadata().iter().map(|(field, _type, meta)| {
                    let sql_type = match *meta {
                        "autoincrement" => "INT AUTO_INCREMENT PRIMARY KEY",
                        _ => meta,
                    };
                    format!("{} {}", field, sql_type)
                }).collect::<Vec<String>>().join(",\n    ");
                
                format!("CREATE TABLE {} (\n    {}\n);", $table_name, columns)
            }
        }
    };
}


/////// ==== Exemplo 4 =============
#[macro_export]
macro_rules! create_struct_and_metadata_com_sql_methods {
    ($table_name:expr => $struct_name:ident { $($field_name:ident: $field_type:ty, $metadata:expr),* $(,)? }) => {
        struct $struct_name {
            $(pub $field_name: $field_type,)*
        }
        
        impl $struct_name {
            pub fn metadata() -> Vec<(&'static str, &'static str, &'static str)> {
                vec![
                    $( (stringify!($field_name), stringify!($field_type), $metadata), )*
                ]
            }

            pub fn generate_sql_create_table() -> String {
                let columns = Self::metadata().iter().map(|(field, _type, meta)| {
                    let sql_type = match *meta {
                        "autoincrement" => "INT AUTO_INCREMENT PRIMARY KEY",
                        _ => meta,
                    };
                    format!("{} {}", field, sql_type)
                }).collect::<Vec<String>>().join(",\n    ");
                
                format!("CREATE TABLE {} (\n    {}\n);", $table_name, columns)
            }

            pub fn generate_sql_drop_table() -> String {
                format!("DROP TABLE {};", $table_name)
            }

            pub fn generate_sql_insert() -> String {
                let fields = Self::metadata().iter()
                    .map(|(field, _, _)| *field)
                    .filter(|&field| field != "id")
                    .collect::<Vec<_>>()
                    .join(", ");

                let values = Self::metadata().iter()
                    .map(|(field, _, _)| format!(":{}", field))
                    .filter(|field| field != ":id")
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("INSERT INTO {} ({}) VALUES ({});", $table_name, fields, values)
            }

            pub fn generate_sql_update() -> String {
                let updates = Self::metadata().iter().map(|(field, _, _)| format!("{} = :{}", field, field)).filter(|update| update != "id = :id").collect::<Vec<_>>().join(", ");
                format!("UPDATE {} SET {} WHERE id = :id;", $table_name, updates)
            }

            pub fn generate_sql_delete() -> String {
                format!("DELETE FROM {} WHERE id = :id;", $table_name)
            }

            pub fn generate_sql_select() -> String {
                format!("SELECT * FROM {};", $table_name)
            }
        }
    };
}