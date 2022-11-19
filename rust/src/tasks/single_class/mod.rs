pub mod single_data;
pub mod single_arrow;
pub mod runner;
pub mod tokenizer;

use serde::{Serialize, Deserialize};

trait ConfigTypes {
    type Transport;
    type DataType;
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SingleClassConfig{
    pub batch_size:usize,
    pub sequence_length:usize,
    pub tokenizer_name:String
}

impl ConfigTypes for SingleClassConfig {
    type Transport = single_data::SingleClassTransport;
    type DataType = single_data::SingleClassData;
}

