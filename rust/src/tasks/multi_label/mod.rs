pub mod multi_data;
pub mod multi_tokenizer;
pub mod multi_arrow;
pub mod runner;

use serde::{Serialize, Deserialize};

use self::multi_data::{MultiTransport, MultiData};



trait ConfigTypes {
    type Transport;
    type DataType;
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct MultiConfig{
    pub batch_size:usize,
    pub sequence_length:usize,
    pub number_labels:usize,
    pub tokenizer_name:String
}

impl ConfigTypes for MultiConfig {
    type Transport = MultiTransport;
    type DataType = MultiData;
}

