use crate::model_views::home_view::HomeView;
use warp::Reply;

pub fn index() -> impl Reply {
    let reply = HomeView {
        mensagem: "Bem vindo a API com Warp!".to_string(),
    };
    warp::reply::json(&reply)
}