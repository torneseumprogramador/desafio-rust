mod models;
mod ux;
mod negocio;
mod repositorios;
mod config;

use ux::menu;

fn main() {
    // fn main(){
    /*
    - tratamento de erros
    
    proximos
    - traits (interfaces)
    - POO - Polimorfismo
    - Generics
    - ORM trabalhando com Generics
    - uso de ORMs existentes
        - persistencia mysql
        - persistencia postgres
        - persistencia mongodb
        - persistencia sqlite (db local)
    - lifetimes
    - metaprogramação - Macros ...
    - Testes
    - Programação funcional vs POO vs programação estruturada
    - Concorrencia e paralelismo
    - Programação Asyncrona
    - Programação Web
        - render server side
        - API's
    - Docs
    - redis - Cache ou chave/valor
    - elasticsearch - Indexação e busca
    */

    menu::carregar();

    // println!("===={}===", config::configuration::get_json_db_alunos_path());
    // println!("===={}===", config::configuration::get_mysql_string_connection());
}