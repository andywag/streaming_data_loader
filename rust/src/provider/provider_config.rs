use serde::{Deserialize, Serialize};

use super::{pile_datasets::PileDatasetType, source_filter::SourceFilter};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum ProviderLength {
    #[serde(rename = "iterations")]
    Iterations{iterations:usize},
    #[serde(rename = "epochs")]
    Epochs{epochs:usize}
}


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct HuggingDescription {
    pub dataset:String,
    pub args:Option<String>,
    pub operations:Vec<String>
}


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct WikiDescription {
    pub location:String,
    pub network:bool
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PileFullDescription {
    pub locations:Vec<String>,
    pub network:bool
}



#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Dataset {
    pub location:String,    
}


#[derive(Deserialize, Serialize, Debug)]
pub struct Datasets {
    pub datasets:Vec<Dataset>
}


#[derive(Deserialize, Serialize, Debug, Clone)]
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

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ProviderConfig {
    pub shuffle:Option<bool>, // Shuffle the data
    pub flatten:Option<bool>, // Load all the data into memory
    pub length:ProviderLength,
    pub source:SourceDescription,
    pub filter:Option<SourceFilter>
}

pub enum Examples {
    Mask,
    Squad,
    Emot,
    Imdb,
    Python
}

impl Examples {
    pub fn get_config(&self, test:bool) -> ProviderConfig {
        if test {
            match self {
                Examples::Mask => ProviderConfig {
                    shuffle: None,
                    flatten: None,
                    length: ProviderLength::Iterations { iterations: 10 },
                    source: SourceDescription::DataList(vec![Dataset{location:"../data/test.json.gz".to_string()}]),
                    filter: None,
                },
                Examples::Squad => {
                    ProviderConfig {
                        shuffle: Some(true),
                        flatten: Some(true),
                        length: ProviderLength::Iterations { iterations:1024 },
                        source: SourceDescription::HuggingFace(HuggingDescription { dataset:"squad".to_string(), args: None, operations: vec![] }),
                        filter: None,
                    }
                },
                Examples::Emot => {
                    ProviderConfig {
                        shuffle: Some(true),
                        flatten: Some(true),
                        length: ProviderLength::Iterations { iterations:1024 },
                        source: SourceDescription::HuggingFace(HuggingDescription { dataset:"xed_en_fi".to_string(), 
                            args: Some("en_annotated".to_string()), operations: vec!["train".to_string()] }),
                        filter: None,
                    }
                },
                Examples::Imdb => {
                    ProviderConfig {
                        shuffle: Some(true),
                        flatten: Some(true),
                        length: ProviderLength::Iterations { iterations:1024 },
                        source: SourceDescription::HuggingFace(HuggingDescription { dataset:"imdb".to_string(), 
                            args: None, operations: vec!["train".to_string()]}),
                        filter: None,
                    }
                },
                Examples::Python => {
                    ProviderConfig {
                        shuffle: Some(true),
                        flatten: Some(true),
                        length: ProviderLength::Epochs { epochs:3 },
                        source: SourceDescription::Pile { typ:PileDatasetType::Total },
                        filter: Some(SourceFilter::PythonText),
                    }
                },
            }
        }
        else {
            match self {
                Examples::Mask => ProviderConfig {
                    shuffle: None,
                    flatten: None,
                    length: ProviderLength::Epochs { epochs : 1 },
                    source: SourceDescription::Pile { typ:PileDatasetType::Total },
                    filter: None,
                },
                Examples::Squad => {
                    ProviderConfig {
                        shuffle: Some(true),
                        flatten: Some(true),
                        length: ProviderLength::Epochs { epochs:3 },
                        source: SourceDescription::HuggingFace(HuggingDescription { dataset:"squad".to_string(), args: None, operations: vec![] }),
                        filter: None,
                    }
                },
                Examples::Emot => {
                    ProviderConfig {
                        shuffle: Some(true),
                        flatten: Some(true),
                        length: ProviderLength::Epochs { epochs:3 },
                        source: SourceDescription::HuggingFace(HuggingDescription { dataset:"xed_en_fi".to_string(), 
                            args: Some("en_annotated".to_string()), operations: vec!["train".to_string()] }),
                        filter: None,
                    }
                },
                Examples::Imdb => {
                    ProviderConfig {
                        shuffle: Some(true),
                        flatten: Some(true),
                        length: ProviderLength::Epochs { epochs:3 },
                        source: SourceDescription::HuggingFace(HuggingDescription { dataset:"imdb".to_string(), 
                            args: None, operations: vec!["train".to_string()]}),
                        filter: None,
                    }
                },
                Examples::Python => {
                    ProviderConfig {
                        shuffle: Some(true),
                        flatten: Some(true),
                        length: ProviderLength::Epochs { epochs:3 },
                        source: SourceDescription::Pile { typ:PileDatasetType::Total },
                        filter: Some(SourceFilter::PythonText),
                    }
                },
            }
        }
    }
}
