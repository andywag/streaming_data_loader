pub mod multi_data;
pub mod multi_tokenizer;
pub mod multi_arrow;
pub mod multi_cases;
pub mod runner;

use serde::{Serialize, Deserialize};

use self::multi_data::{MultiTransport, MultiData};



trait ConfigTypes {
    type Transport;
    type DataType;
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct MultiConfig{
    pub number_labels:usize,
}

impl ConfigTypes for MultiConfig {
    type Transport = MultiTransport;
    type DataType = MultiData;
}

