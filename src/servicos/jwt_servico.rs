use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Serialize, Deserialize};
use chrono::{Utc, Duration};
use crate::models::usuario::Usuario;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String, // Subject (usuário identificado pelo token)
    exp: usize, // Expiry (tempo de expiração do token)
}

pub fn gerar(usuario: &Usuario) -> String {
    let expiration_time = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: usuario.id.to_string(),
        exp: expiration_time as usize,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret("DesafioRustRocketJwt".as_ref())).unwrap()
}

pub fn validar(token: &str) -> bool {
    decode::<Claims>(token, &DecodingKey::from_secret("DesafioRustRocketJwt".as_ref()), &Validation::default()).is_ok()
}