mod models;
mod ux;
mod negocio;
mod repositorios;
mod config;

use ux::menu;

fn main() {
    // fn main(){
    /*
    amanha
    - persistencia
    */

    menu::carregar();

    // println!("===={}===", config::configuration::get_json_db_alunos_path());
    // println!("===={}===", config::configuration::get_mysql_string_connection());
}