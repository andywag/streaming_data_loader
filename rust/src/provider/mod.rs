use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::tasks::DatasetInfo;

use self::pile_datasets::{PileDatasetType};


pub mod provider_util;
//pub mod wiki_file_provider;
//pub mod pile_file_provider;
pub mod arrow_provider;
pub mod arrow_transfer;

pub mod pile_datasets;
pub mod general_file_provider;
pub mod gzip_file_provider;
pub mod zstd_file_provider;

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
pub struct WikiDescription {
    pub location:String,
    pub network:bool
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PileFullDescription {
    pub locations:Vec<String>,
    pub network:bool
}



#[derive(Deserialize, Serialize, Debug)]
pub struct Dataset {
    pub location:String,
    pub network:bool
    
}


#[derive(Deserialize, Serialize, Debug)]
pub struct Datasets {
    pub datasets:Vec<Dataset>
}


#[derive(Deserialize, Serialize, Debug)]
pub enum SourceDescription {
    #[serde(rename = "huggingface")]
    HuggingFace(HuggingDescription),
    #[serde(rename = "pile")]
    Pile{typ:PileDatasetType},
    #[serde(rename = "arrow")]
    Arrow(String),
    #[serde(rename="list")]
    DataList(Vec<Dataset>)

}

#[derive(Deserialize, Serialize, Debug)]
pub struct ProviderConfig {
    pub shuffle:Option<bool>, // Shuffle the data
    pub flatten:Option<bool>, // Load all the data into memory
    pub length:ProviderLength,
    pub source:SourceDescription
}


