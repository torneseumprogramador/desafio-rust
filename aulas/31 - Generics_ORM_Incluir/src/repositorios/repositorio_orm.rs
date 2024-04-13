use mysql::{Pool, PooledConn};
use mysql::prelude::*;
use crate::traits::entidade::TEntidade;
use inflector::Inflector;
use serde_json::Value;
use mysql::serde::Serialize;
use std::marker::PhantomData;

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

    pub fn criar_tabela(&self) {
        let mut conn = self.get_conn();
        let nome_tabela = self.nome_tabela();
        let detalhe_dos_campos_tabela = self.detalhe_dos_campos_tabela();

        let sql = format!("CREATE TABLE IF NOT EXISTS {} ({})", nome_tabela, detalhe_dos_campos_tabela);

        println!("=============");
        println!("{}", sql);
        println!("=============");

        conn.exec_drop(sql, {}).expect("Failed to create table.");
    }

    pub fn incluir(&self, model: &T) -> i32 {
        let mut conn = self.get_conn();
        let nome_tabela = self.nome_tabela();
        let campos = self.campos_tabela("".to_string());
        let key_campos = self.campos_tabela(":".to_string());
        let valores = self.extrair_valores(model);
        
        let sql = format!("INSERT INTO {} ({}) values ({})", nome_tabela, campos, key_campos);

        println!("=============");
        println!("{}", sql);
        println!("=============");
        println!("{:?}", valores);
        println!("=============");

        conn.exec_drop(sql, valores).expect("Failed to insert data.");
        
        conn.last_insert_id() as i32
    }
}
