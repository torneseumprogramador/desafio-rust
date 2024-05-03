use rocket_dyn_templates::tera::{Result as TeraResult, Value, Error as TeraError};
use std::collections::HashMap;

pub fn media(value: &Value, _: &HashMap<String, Value>) -> TeraResult<Value> {
    let notas = value.as_array().ok_or(TeraError::msg("Expected an array"))?;
    let soma: f64 = notas.iter().map(|v| v.as_f64().unwrap_or(0.0)).sum();
    let media = soma / notas.len() as f64;
    Ok(Value::from(media))
}

pub fn situacao(value: &Value, _: &HashMap<String, Value>) -> TeraResult<Value> {
    let media = value.as_f64().ok_or(TeraError::msg("Expected a float"))?;
    let situacao = if media < 5.0 {
        "reprovação"
    } else if media < 7.0 {
        "recuperação"
    } else {
        "aprovado"
    };
    Ok(Value::from(situacao))
}
