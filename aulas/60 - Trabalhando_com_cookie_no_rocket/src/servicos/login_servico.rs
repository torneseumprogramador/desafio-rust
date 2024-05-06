use crate::dtos::login_dto::LoginDto;

pub fn logar(login_dto: LoginDto) -> Result<(), String>{
    if login_dto.email.is_empty() {
        return Err(String::from("O email é obrigatório"));
    }

    if login_dto.senha.is_empty() {
        return Err(String::from("A senha é obrigatória"));
    }
    
    // TODO caso vc queira, faça a validação no repo ORM aqui

    if login_dto.email != String::from("danilo@teste.com") && login_dto.senha != String::from("123456")  {
        return Err(String::from("Email ou senha inválidos"));
    }
    
    Ok(())
}
