pub mod multi_data;
pub mod multi_tokenizer;
pub mod multi_test_endpoint;
pub mod multi_arrow;
pub mod multi_runner;

use serde::{Serialize, Deserialize};

use self::multi_data::{MultiTransport, MultiData};



trait ConfigTypes {
    type Transport;
    type DataType;
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MultiConfig{
    pub batch_size:u32,
    pub sequence_length:u32,
    pub number_labels:u32,
    pub tokenizer_name:String
}

impl ConfigTypes for MultiConfig {
    type Transport = MultiTransport;
    type DataType = MultiData;
}

