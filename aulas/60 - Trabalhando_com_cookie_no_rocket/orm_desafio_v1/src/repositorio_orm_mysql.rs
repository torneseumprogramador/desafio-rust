use mysql::{prelude::*, Pool, PooledConn, Params, Row};
use std::marker::PhantomData;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::traits::TEntidade;
use mysql::Value as MySqlValue;
use std::fmt::Debug;

pub struct RepositorioOrmMySql<T: TEntidade + Debug + Serialize + for<'de> Deserialize<'de> + Default> {
    pool: Pool,
    _marker: PhantomData<T>,
}

impl<T: TEntidade + Debug + Serialize + for<'de> Deserialize<'de> + Default> RepositorioOrmMySql<T> {
    pub fn new(str_connection: &str) -> Self {
        RepositorioOrmMySql {
            pool: Pool::new(str_connection).unwrap(),
            _marker: PhantomData,
        }
    }

    fn get_conn(&self) -> PooledConn {
        self.pool.get_conn().unwrap()
    }

    pub fn apagar_tabela(&self) {
        let mut conn = self.get_conn();
        conn.exec_drop(T::generate_sql_drop_table(), ()).expect("Failed to drop table.");
    }

    pub fn criar_tabela(&self) {
        let mut conn = self.get_conn();
        conn.exec_drop(T::generate_sql_create_table(), ()).expect("Failed to create table.");
    }

    fn process_row(&self, row: Row) -> T {
        let mut values = Vec::new();
        for value in row.clone().unwrap().into_iter() {
            values.push(value);
        }
    
        let column_names = row.columns_ref().iter().map(|col| col.name_str().to_string()).collect::<Vec<_>>();
        
        let json_str = column_names.into_iter().zip(values.into_iter()).map(|(col, val)| {
            let value_str = match val {
                mysql::Value::Bytes(s) => format!("\"{}\"", String::from_utf8_lossy(&s).replace("\"", "\\\"")), // Corrige aspas para strings.
                mysql::Value::Int(i) => i.to_string(), // Manipula inteiros diretamente.
                mysql::Value::Float(f) => f.to_string(), // Manipula floats diretamente.
                _ => val.as_sql(true) // Usa representação SQL para outros tipos de dados.
            };
            format!("\"{}\":{}", col, value_str)
        }).collect::<Vec<_>>().join(",");
        
        let json_data = format!("{{ {} }}", json_str);
        
        let obj: Result<T, _> = serde_json::from_str(&json_data);
        match obj {
            Ok(obj) => obj,
            Err(e) => {
                eprintln!("Erro ao desserializar: {}", e);
                Default::default() // Retorna o valor default para T se falhar
            }
        }
    }

    fn extrair_valores(&self, model: &T) -> Vec<(String, mysql::Value)> {
        let model_json = serde_json::to_value(model).unwrap();
        match model_json {
            serde_json::Value::Object(map) => {
                map.into_iter().map(|(key, val)| {
                    let mysql_val = match val {
                        serde_json::Value::String(s) => mysql::Value::from(s),
                        serde_json::Value::Number(num) => {
                            if let Some(int) = num.as_i64() {
                                mysql::Value::from(int)
                            } else if let Some(float) = num.as_f64() {
                                mysql::Value::from(float)
                            } else {
                                mysql::Value::NULL
                            }
                        },
                        serde_json::Value::Bool(b) => mysql::Value::from(b),
                        _ => mysql::Value::NULL,
                    };
                    (key, mysql_val)
                }).collect()
            },
            _ => panic!("Expected an object for serialization of model values."),
        }
    }

    pub fn todos(&self) -> Vec<T> {
        self.where_query(String::new(), &HashMap::new())
    }

    pub fn buscar_por_id(&self, id: i32) -> Option<T> {
        let result = self.where_query(format!("id = {}", id), &HashMap::new());
        result.into_iter().next()
    }

    pub fn where_query(&self, where_sql: String, params: &HashMap<String, String>) -> Vec<T> {
        let mut conn = self.get_conn();
        let mut sql = T::generate_sql_select();

        let result: Vec<T> = if params.is_empty() {
            conn.exec_map(
                &sql,
                (),
                |row| self.process_row(row)
            ).expect("Failed to retrieve data.")
        } else {
            if !where_sql.is_empty() {
                sql.push_str(" where 1=1 and ");
                sql.push_str(&where_sql);
            }
    
            let mysql_params: Vec<(String, MySqlValue)> = params.into_iter()
                .map(|(k, v)| (k.clone(), mysql::Value::from(v)))
                .collect();

            conn.exec_map(
                &sql,
                Params::from(mysql_params),
                |row| self.process_row(row)
            ).expect("Failed to retrieve data.")
        };

        result
    }

    pub fn apagar_por_id(&self, id: i32) {
        let mut params = HashMap::new();
        params.insert("id".to_string(), id.to_string());
        self.apagar_where("id = :id".to_string(), &params)
    }

    pub fn apagar_where(&self, where_sql: String, params: &HashMap<String, String>) {
        let mut conn = self.get_conn();
        let mut sql = T::generate_sql_delete();
        
        if params.is_empty() {
            conn.exec_drop(&sql, ()).expect("Failed to delete data.");
        } else {
            if !where_sql.is_empty() {
                sql.push_str(" where 1=1 and ");
                sql.push_str(&where_sql);
            }
    
            let mysql_params: Vec<(String, MySqlValue)> = params.into_iter()
                .map(|(k, v)| (k.clone(), mysql::Value::from(v)))
                .collect();

            conn.exec_drop(&sql,Params::from(mysql_params)).expect("Failed to delete data.")
        }
    }

    pub fn atualizar(&self, model: &T) {
        let mut conn = self.get_conn();
        let sql = T::generate_sql_update();
    
        let params = self.extrair_valores(model);
        let mysql_params = Params::from(params);
    
        conn.exec_drop(sql, mysql_params).expect("Failed to update data.");
    }

    pub fn incluir(&self, model: &T) -> i32 {
        let mut conn = self.get_conn();
        let sql = T::generate_sql_insert();

        let params = self.extrair_valores(model);
        let mysql_params = Params::from(params);

        conn.exec_drop(sql, mysql_params).expect("Failed to insert data.");
        conn.last_insert_id() as i32
    }

    pub fn count(&self) -> i32 {
        let mut conn = self.get_conn();
        let sql = T::generate_sql_select_count();
        let result: Option<i32> = conn.query_first(sql).expect("Query failed.");
        result.expect("No count result found.")
    }

    pub fn exec_sql_to_hashmap_vec(&self, sql: &str) -> Result<Vec<HashMap<String, String>>, mysql::Error> {
        let mut conn = self.pool.get_conn()?;

        let selected_rows: Vec<Row> = conn.query(sql)?;

        let mut result_vec = Vec::new();
        for row in selected_rows.into_iter() {
            let columns = row.columns_ref();
            let mut row_map = HashMap::new();

            let cloned_row = row.clone();
            let values = cloned_row.unwrap();

            for (idx, value) in values.into_iter().enumerate() {
                let column_name = columns[idx].name_str().to_string();
                let value_str = match value {
                    mysql::Value::Bytes(v) => String::from_utf8_lossy(&v).to_string(),
                    mysql::Value::Int(i) => i.to_string(),
                    mysql::Value::Float(f) => f.to_string(),
                    mysql::Value::NULL => "NULL".to_string(),
                    mysql::Value::Date(year, month, day, hour, minute, second, micro) =>
                        format!("{:04}-{:02}-{:02} {:02}:{:02}:{:02}.{:06}", year, month, day, hour, minute, second, micro),
                    mysql::Value::Time(negative, days, hours, minutes, seconds, microseconds) =>
                        format!("{}{} day(s) {:02}:{:02}:{:02}.{:06}", if negative { "-" } else { "" }, days, hours, minutes, seconds, microseconds),
                    _ => "Unsupported type".to_string(),
                };

                row_map.insert(column_name, value_str);
            }

            result_vec.push(row_map);
        }

        Ok(result_vec)
    }
}
