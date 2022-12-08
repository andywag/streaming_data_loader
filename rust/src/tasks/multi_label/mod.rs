pub mod multi_arrow;
pub mod multi_cases;

use serde::{Serialize, Deserialize};




trait ConfigTypes {
    type Transport;
    type DataType;
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct MultiConfig{
    pub number_labels:usize,
}


