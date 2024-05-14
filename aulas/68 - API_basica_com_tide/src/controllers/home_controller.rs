use tide::{Request, Response, Result};
use crate::model_views::home_view::HomeView;
use serde_json::to_string;

pub async fn index(_req: Request<()>) -> Result {
    let home_view = HomeView {
        mensagem: "Olá Alunos estamos no Tide com Json".to_string(),
    };

    let message = to_string(&home_view)?;
    
    Ok(Response::builder(200)
        .body(message)
        .content_type(tide::http::mime::JSON)
        .build())
}