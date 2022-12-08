pub mod squad_data;
pub mod squad_arrow;
pub mod squad_tokenizer;
pub mod squad_endpoint;
pub mod squad_cases;


use serde::{Serialize, Deserialize};


#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SquadConfig{
}