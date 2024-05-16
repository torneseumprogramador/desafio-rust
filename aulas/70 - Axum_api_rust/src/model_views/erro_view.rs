use serde::Serialize;

#[derive(Serialize)]
pub struct ErroView {
    pub mensagem: String,
}