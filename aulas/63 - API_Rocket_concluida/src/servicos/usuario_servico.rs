use crate::models::usuario::Usuario;
use crate::orm_desafio_v1::repositorio_orm_mysql::RepositorioOrmMySql;
use crate::config::configuration;
use bcrypt::{hash, DEFAULT_COST};

pub fn incluir(usuario: Usuario) -> Result<i32, String> {
    if usuario.nome.is_empty() {
        return Err(String::from("O nome é obrigatório"));
    }

    if usuario.email.is_empty() {
        return Err(String::from("A email é obrigatória"));
    }

    if usuario.senha.is_empty() {
        return Err(String::from("A senha é obrigatória"));
    }

    let senha_cripto = criptografa_senha(&usuario.senha);

    let usuario_senha_criptografada = Usuario {
        id: 0,
        nome: usuario.nome,
        email: usuario.email,
        senha: senha_cripto
    };

    let aluno_id = repo().incluir(&usuario_senha_criptografada);
    Ok(aluno_id)
}

fn criptografa_senha(senha: &str) -> String {
    hash(senha, DEFAULT_COST).unwrap()
}

fn repo() -> RepositorioOrmMySql::<Usuario> {
    let sql_connection = configuration::get_mysql_string_connection();
    return RepositorioOrmMySql::<Usuario>::new(&sql_connection);
}
