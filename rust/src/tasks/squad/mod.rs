pub mod squad_data;
//pub mod squad_arrow;
pub mod squad_arrow;
pub mod squad_tokenizer;
//pub mod squad_top;

pub mod squad_endpoint;

pub mod runner;

use serde::{Serialize, Deserialize};


#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SquadConfig{
}