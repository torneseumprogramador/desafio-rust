
#[derive(FromForm)]
pub struct LoginDto {
    pub email: String,
    pub senha: String
}