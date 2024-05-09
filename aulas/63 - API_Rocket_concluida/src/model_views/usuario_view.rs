use serde::{Serialize};

#[derive(Debug, Default, Serialize)]
pub struct UsuarioView {
    pub id: i32,
    pub nome: String,
    pub email: String,
    pub token: String,
}