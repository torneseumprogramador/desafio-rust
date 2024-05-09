
#[derive(serde::Deserialize)]
pub struct LoginDto {
    pub email: String,
    pub senha: String
}