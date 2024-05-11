#[macro_export]
macro_rules! create_struct_and_metadata_com_sql_methods {
    ($table_name:expr => $struct_name:ident { $($field_name:ident: $field_type:ty, $metadata:expr),* $(,)? }) => {
        #[derive(Debug, Default, Serialize, Deserialize, Clone)]
        pub struct $struct_name {
            $(pub $field_name: $field_type,)*
        }

        impl TEntidade for $struct_name {
            fn id(&self) -> i32 {
                self.id
            }

            fn metadata() -> Vec<(&'static str, &'static str, &'static str)> {
                vec![
                    $( (stringify!($field_name), stringify!($field_type), $metadata), )*
                ]
            }

            fn generate_sql_create_table() -> String {
                let columns = Self::metadata().iter().map(|(field, _type, meta)| {
                    let sql_type = match *meta {
                        "autoincrement" => "INT AUTO_INCREMENT PRIMARY KEY",
                        _ => meta,
                    };
                    format!("{} {}", field, sql_type)
                }).collect::<Vec<String>>().join(",\n    ");
                
                format!("CREATE TABLE IF NOT EXISTS {} (\n    {}\n);", $table_name, columns)
            }

            fn generate_sql_drop_table() -> String {
                format!("DROP TABLE IF EXISTS {};", $table_name)
            }

            fn generate_sql_insert() -> String {
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

            fn generate_sql_update() -> String {
                let updates = Self::metadata().iter().map(|(field, _, _)| format!("{} = :{}", field, field)).filter(|update| update != "id = :id").collect::<Vec<_>>().join(", ");
                format!("UPDATE {} SET {} WHERE id = :id;", $table_name, updates)
            }

            fn generate_sql_delete() -> String {
                format!("DELETE FROM {}", $table_name)
            }

            fn generate_sql_select() -> String {
                format!("SELECT * FROM {}", $table_name)
            }

            fn generate_sql_select_count() -> String {
                format!("SELECT count(1) FROM {}", $table_name)
            }
        }
    };
}
