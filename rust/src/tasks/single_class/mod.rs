pub mod single_data;
pub mod single_arrow;
pub mod runner;
pub mod tokenizer;
pub mod single_cases;

use serde::{Serialize, Deserialize};

trait ConfigTypes {
    type Transport;
    type DataType;
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SingleClassConfig{}

impl ConfigTypes for SingleClassConfig {
    type Transport = single_data::SingleClassTransport;
    type DataType = single_data::SingleClassData;
}

