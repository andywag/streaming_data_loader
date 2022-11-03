pub mod squad_data;
//pub mod squad_arrow;
pub mod squad_arrow;
pub mod squad_tokenizer;
//pub mod squad_top;

pub mod squad_endpoint;

pub mod runner;

use serde::{Serialize, Deserialize};


#[derive(Clone, Serialize, Deserialize)]
pub struct SquadConfig{
    pub batch_size:u32,
    pub sequence_length:u32,
    pub tokenizer_name:String
}