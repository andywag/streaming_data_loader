use serde::{Serialize,Deserialize};

pub mod masking;
pub mod multi_label;
pub mod squad;

pub mod generic_runner;

#[derive(Debug, Serialize, Deserialize)]
pub struct DatasetInfo {
    pub name:String,
    pub length:u32
}