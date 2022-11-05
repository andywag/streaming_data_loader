use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::tasks::DatasetInfo;



pub mod wiki_file_provider;
pub mod arrow_provider;
pub mod arrow_transfer;

pub enum ProviderChannel<T> {
    Complete,
    Info(DatasetInfo),
    Data(T)
}


#[derive(Deserialize, Serialize, Debug)]
pub enum ProviderLength {
    #[serde(rename = "iterations")]
    Iterations{iterations:usize},
    #[serde(rename = "epochs")]
    Epochs{epochs:usize}
}


#[derive(Deserialize, Serialize, Debug)]
pub struct HuggingDescription {
    pub dataset:String,
    pub args:Option<String>,
    pub operations:Vec<String>,
    pub connections:Option<HashMap<String,String>>
}

#[derive(Deserialize, Serialize, Debug)]
pub enum SourceDescription {
    #[serde(rename = "huggingface")]
    HuggingFace(HuggingDescription),
    #[serde(rename = "arrow")]
    Arrow(String),
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ProviderConfig {
    pub shuffle:Option<bool>, // Shuffle the data
    pub flatten:Option<bool>, // Load all the data into memory
    pub length:ProviderLength,
    pub source:SourceDescription
}


