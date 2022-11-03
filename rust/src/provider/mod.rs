use serde::{Deserialize, Serialize};

use crate::datasets::DatasetInfo;



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
pub struct ProviderConfig {
    pub shuffle:Option<bool>, // Shuffle the data
    pub flatten:Option<bool>, // Load all the data into memory
    pub length:ProviderLength
}


/* 
pub struct ProviderConfigIterations {
    pub iterations:u64
}
pub struct ProviderConfigEpochs {
    pub epochs:u64,
}

pub enum ProviderConfig {
    Iterations(ProviderConfigIterations),
    Epochs(ProviderConfigEpochs)
}
*/