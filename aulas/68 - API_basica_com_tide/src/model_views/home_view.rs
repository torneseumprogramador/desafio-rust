use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct HomeView {
    pub mensagem: String
}