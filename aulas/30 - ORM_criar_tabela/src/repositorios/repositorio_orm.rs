use mysql::{Pool, PooledConn};
use mysql::prelude::*;
use crate::traits::entidade::TEntidade;

use std::any::type_name;

pub struct RespositorioORM {
    pool: Pool,
}

impl RespositorioORM {
    pub fn new(str_connection: &str) -> Self {
        RespositorioORM {
            pool: Pool::new(str_connection).unwrap(),
        }
    }

    fn get_conn(&self) -> PooledConn {
        self.pool.get_conn().unwrap()
    }

    fn nome_tabela<T: TEntidade>(&self, model: &T) -> String {
        let nome_tabela = type_name::<T>().split("::").last().unwrap_or_default();
        let nome_tabela = format!("{}s", nome_tabela.to_lowercase());
        nome_tabela
    }

    // TODO Resolver com  Macro
    // fn campos_model<T: TEntidade>(&self, model: &T) -> Vec<String> {
    //     let mut nome_campos: Vec<String> = Vec::new();

    //     nome_campos.push("nome".to_string());
    //     nome_campos.push("matricula".to_string());

    //     nome_campos
    // }

    fn detalhe_dos_campos_tabela<T: TEntidade>(&self, model: &T) -> String {
        

        let mut sql = String::from("");

        let nome_campos = model.campos_model();

        sql.push_str("id int AUTO_INCREMENT, ");

        for campo in nome_campos {
            sql.push_str(&format!("{} varchar(255), ", campo));
        }

        sql.push_str("PRIMARY KEY (id)");

        sql
    }

    pub fn criar_tabela<T: TEntidade>(&self, model: T) {
        let mut conn = self.get_conn();

        let nome_tabela = self.nome_tabela(&model);
        let detalhe_dos_campos_tabela = self.detalhe_dos_campos_tabela(&model);

        let sql = format!("CREATE TABLE IF NOT EXISTS {} ({})", nome_tabela, detalhe_dos_campos_tabela);

        println!("=============");
        println!("{}", sql);
        println!("=============");

        conn.exec_drop(sql, {}).unwrap();
    }
}
