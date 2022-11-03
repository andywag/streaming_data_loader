pub mod single_data;
pub mod single_arrow;
pub mod runner;
pub mod tokenizer;

use serde::{Serialize, Deserialize};

trait ConfigTypes {
    type Transport;
    type DataType;
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SingleClassConfig{
    pub batch_size:u32,
    pub sequence_length:u32,
    pub number_labels:u32,
    pub tokenizer_name:String
}

impl ConfigTypes for SingleClassConfig {
    type Transport = single_data::SingleClassTransport;
    type DataType = single_data::SingleClassData;
}

