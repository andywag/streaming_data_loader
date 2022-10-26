//pub mod multi_top;

use serde::{Serialize, Deserialize};



#[derive(Clone, Serialize, Deserialize)]
pub struct SquadConfig{
    pub batch_size:u32,
    pub sequence_length:u32,
    pub tokenizer_name:String
}

