use mysql::{prelude::*, Pool, PooledConn, Row, Params};
use crate::traits::entidade::TEntidade;
use inflector::Inflector;
use serde_json::Value;
use mysql::serde::Serialize;
use std::marker::PhantomData;
use std::collections::HashMap;
use mysql::Value as MySqlValue;

use std::any::type_name;

pub struct RepositorioORM<T: TEntidade + Serialize> {
    pool: Pool,
    _marker: PhantomData<T>,
}

impl<T: TEntidade + Serialize> RepositorioORM<T> {
    pub fn new(str_connection: &str) -> Self {
        RepositorioORM {
            pool: Pool::new(str_connection).unwrap(),
            _marker: PhantomData,
        }
    }

    fn get_conn(&self) -> PooledConn {
        self.pool.get_conn().unwrap()
    }

    fn nome_tabela(&self) -> String {
        type_name::<T>().split("::").last().unwrap_or_default().to_snake_case() + "s"
    }

    fn campos_tabela(&self, key: String) -> String {
        T::campos_model().into_iter()
            .map(|(nome, _)| format!("{}{}", key, nome))
            .collect::<Vec<String>>()
            .join(", ")
    }

    fn campos_com_keys_para_updade(&self) -> String {
        T::campos_model().into_iter()
            .map(|(nome, _)| format!("{}=:{}", nome, nome))
            .collect::<Vec<String>>()
            .join(", ")
    }

    fn detalhe_dos_campos_tabela(&self) -> String {
        let campos = T::campos_model().into_iter()
            .map(|(nome, tipo)| format!("{} {}", nome, tipo))
            .collect::<Vec<String>>()
            .join(", ");

        format!("id int AUTO_INCREMENT, {}, PRIMARY KEY (id)", campos)
    }

    fn extrair_valores(&self, model: &T) -> Vec<(String, mysql::Value)> {
        let model_json = serde_json::to_value(model).unwrap();
        match model_json {
            Value::Object(map) => {
                map.into_iter().map(|(key, val)| {
                    let mysql_val = match val {
                        Value::String(s) => mysql::Value::from(s),
                        Value::Number(num) => {
                            if let Some(int) = num.as_i64() {
                                mysql::Value::from(int)
                            } else if let Some(float) = num.as_f64() {
                                mysql::Value::from(float)
                            } else {
                                mysql::Value::NULL
                            }
                        },
                        Value::Bool(b) => mysql::Value::from(b),
                        _ => mysql::Value::NULL,
                    };
                    (key, mysql_val)
                }).collect()
            },
            _ => panic!("Esperado um objeto para serialização dos valores do modelo."),
        }
    }

    fn convert_mysql_value_to_json(&self, value: MySqlValue) -> Value {
        match value {
            MySqlValue::NULL => Value::Null,
            MySqlValue::Int(val) => Value::Number(serde_json::Number::from(val)),
            MySqlValue::UInt(val) => {
                // Note que converter um u32 para i64 é seguro, mas pode haver perda de precisão se o valor for muito grande
                Value::Number(serde_json::Number::from(val as i64))
            },
            MySqlValue::Float(val) => {
                // Converta de f32 para f64 antes de passar para serde_json::Number
                Value::Number(serde_json::Number::from_f64(val as f64).unwrap_or_else(|| serde_json::Number::from(0)))  // Handle NaN or Infinity
            },
            MySqlValue::Bytes(val) => {
                // Aqui estamos assumindo que os bytes são UTF-8; lidar com erros pode ser necessário
                Value::String(String::from_utf8(val).unwrap_or_default())
            },
            MySqlValue::Date(year, month, day, hour, min, sec, micro) => {
                let date_string = format!("{}-{:02}-{:02}T{:02}:{:02}:{:02}.{:06}",
                                          year, month, day, hour, min, sec, micro);
                Value::String(date_string)  // Simplified date-time string
            },
            _ => Value::Null  // Caso padrão para outros tipos de valores não tratados
        }
    }

    fn convert_json_to_mysql(&self, value: &Value) -> MySqlValue {
        match value {
            Value::Null => MySqlValue::NULL,
            Value::Number(num) => {
                if let Some(int) = num.as_i64() {
                    MySqlValue::Int(int)
                } else if let Some(float) = num.as_f64() {
                    MySqlValue::Float(float as f32) // Convertendo f64 para f32, pode precisar de ajustes baseados na precisão
                } else {
                    MySqlValue::NULL
                }
            },
            Value::String(str) => MySqlValue::Bytes(str.clone().into_bytes()),
            Value::Bool(b) => MySqlValue::from(*b),
            _ => MySqlValue::NULL // Adicione mais conversões conforme necessário
        }
    }

    fn process_row(&self, row: Row) -> Box<T> {
        let column_names: Vec<String> = row.columns_ref().iter()
            .map(|col| col.name_str().to_string())
            .collect();
    
        let values: Vec<Value> = row.unwrap().into_iter()
            .map(|val| self.convert_mysql_value_to_json(val))
            .collect();
    
        let row_map: HashMap<String, Value> = column_names.into_iter()
            .zip(values.into_iter())
            .collect();
    
        T::from_data_row(row_map).unwrap()
    }

    pub fn criar_tabela(&self) {
        let mut conn = self.get_conn();
        let nome_tabela = self.nome_tabela();
        let detalhe_dos_campos_tabela = self.detalhe_dos_campos_tabela();

        let sql = format!("CREATE TABLE IF NOT EXISTS {} ({})", nome_tabela, detalhe_dos_campos_tabela);

        conn.exec_drop(sql, {}).expect("Failed to create table.");
    }

    pub fn todos(&self) -> Vec<T> {
        self.where_query(String::new(), HashMap::new())
    }

    pub fn buscar_por_id(&self, id: i32) -> Option<T> {
        let mut params = HashMap::new();
        params.insert("id".to_string(), Value::Number(id.into()));

        let mut result = self.where_query("id = :id".to_string(), params);

        result.into_iter().next()
    }

    pub fn where_query(&self, where_sql: String, params: HashMap<String, Value>) -> Vec<T> {
        let sql_where_formatado = if !where_sql.is_empty() {
            format!("WHERE {}", where_sql)
        } else {
            String::new()
        };

        let mut conn = self.get_conn();
        let nome_tabela = self.nome_tabela();
        let sql = format!("SELECT * FROM {} {}", nome_tabela, sql_where_formatado);
    
        let mysql_params: Vec<(String, MySqlValue)> = params.into_iter()
        .map(|(k, v)| (k, self.convert_json_to_mysql(&v)))
        .collect();

        let boxed_result: Vec<Box<T>> = if !mysql_params.is_empty() {
            conn.exec_map(
                &sql,
                Params::from(mysql_params),
                |row: Row| {
                    self.process_row(row)
                }
            )
        } else {
            conn.exec_map(
                &sql,
                (),
                |row: Row| {
                    self.process_row(row)
                }
            )
        }.expect("Failed to retrieve data.");
    
        let result: Vec<T> = boxed_result.into_iter().map(|boxed_item| *boxed_item).collect();
        result
    }


    pub fn apagar_por_id(&self, id: i32) {
        let mut conn = self.get_conn();
        let nome_tabela = self.nome_tabela();
        let sql = format!("DELETE FROM {} WHERE id = {}", nome_tabela, id);

        conn.exec_drop(sql, {}).expect("Failed to insert data.");
    }


    pub fn atualizar(&self, model: &T) {
        let mut conn = self.get_conn();
        let nome_tabela = self.nome_tabela();
        let campos_com_keys_para_updade = self.campos_com_keys_para_updade();
        let valores = self.extrair_valores(model);
        
        let sql = format!("UPDATE {} SET {} WHERE id = {}", nome_tabela, campos_com_keys_para_updade, model.id());

        conn.exec_drop(sql, valores).expect("Failed to insert data.");
    }

    pub fn incluir(&self, model: &T) -> i32 {
        let mut conn = self.get_conn();
        let nome_tabela = self.nome_tabela();
        let campos = self.campos_tabela("".to_string());
        let key_campos = self.campos_tabela(":".to_string());
        let valores = self.extrair_valores(model);
        
        let sql = format!("INSERT INTO {} ({}) values ({})", nome_tabela, campos, key_campos);

        conn.exec_drop(sql, valores).expect("Failed to insert data.");
        
        conn.last_insert_id() as i32
    }
}
