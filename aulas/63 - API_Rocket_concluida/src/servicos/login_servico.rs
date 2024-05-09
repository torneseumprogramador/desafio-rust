use crate::dtos::login_dto::LoginDto;
use crate::models::usuario::Usuario;
use crate::orm_desafio_v1::repositorio_orm_mysql::RepositorioOrmMySql;
use crate::config::configuration;
use std::collections::HashMap;
use bcrypt::{verify, BcryptError};

pub fn logar(login_dto: LoginDto) -> Result<Usuario, String>{
    if login_dto.email.is_empty() {
        return Err(String::from("O email é obrigatório"));
    }

    if login_dto.senha.is_empty() {
        return Err(String::from("A senha é obrigatória"));
    }
    
    let sql_connection = configuration::get_mysql_string_connection();
    let repo = RepositorioOrmMySql::<Usuario>::new(&sql_connection);

    let usuarios = repo.where_query("email = :email".to_string(), &HashMap::from([
        ("email".to_string(), login_dto.email),
    ]));

    if usuarios.len() == 0  {
        return Err(String::from("Email ou senha inválidos"));
    }

    let usuario = usuarios[0].clone();
    match senha_valida(&usuario.senha, &login_dto.senha) {
        Ok(validado) => {
            if validado {
                Ok(usuario)
            }
            else {
                Err(String::from("Email ou senha inválidos"))
            }
        },
        Err(_) => Err(String::from("Email ou senha inválidos"))
    }
}

fn senha_valida(senha_db: &str, senha_dto: &str) -> Result<bool, BcryptError> {
    verify(senha_dto, senha_db)
}

