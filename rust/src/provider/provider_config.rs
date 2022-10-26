use core::fmt;

use serde::{Deserialize, de::{Visitor, MapAccess}};

use crate::provider::arrow_provider;

use super::ProviderChannel;


// Constants Used for Decoding YAML File
pub const SOURCE:&str = "source";
pub const ITERATIONS:&str = "iterations";

// Source Types
pub const ARROW:&str = "arrow";
pub const WIKI:&str = "wiki";
pub const HUGGING:&str = "hugging";


#[derive(Debug, Deserialize)]

pub struct ArrowConfig{
    pub location:String
}

#[derive(Debug, Deserialize)]

pub struct WikiConfig {
    pub location:String
}

#[derive(Debug, Deserialize)]

pub struct HuggingConfig {
    pub dataset:String,
    pub arg:Option<String>,
    pub key:String
}


#[derive(Debug)]
pub enum Loader {
    ArrowProvider(ArrowConfig),
    WikiProvider(WikiConfig)
}

pub enum DataSet {
    Squad,
    Masked,
    MultiLabel
}


#[derive(Debug)]
pub struct Source {
    pub iterations:u32,
    pub loader:Loader
}

impl Source {
    pub fn create(config:serde_yaml::Value) -> Self {
        serde_yaml::from_value(config).unwrap()
    }

    pub async fn run_provider(channel:ProviderChannel<DataSet>) {
        
    }
}

impl<'de> Deserialize<'de> for Source {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        deserializer.deserialize_map(SourceVisitor)        
    }
}

struct SourceVisitor;

impl<'de> Visitor<'de> for SourceVisitor {
    type Value = Source;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a map with keys 'first' and 'second'")
    }

    fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>
    {
        let mut iterations:Option<u32> = None;
        let mut loader:Option<Loader> = None;
        //let mut second = None;

        while let Some(k) = map.next_key::<String>()? {
            match k.as_str() {
                ITERATIONS => iterations = map.next_value().unwrap(),
                ARROW => {
                    let value = map.next_value::<serde_yaml::Value>().unwrap();
                    let config = serde_yaml::from_value::<ArrowConfig>(value).unwrap();
                    loader = Some(Loader::ArrowProvider(config));
                }
                WIKI => {
                    let value = map.next_value::<serde_yaml::Value>().unwrap();
                    let config = serde_yaml::from_value::<WikiConfig>(value).unwrap();
                    loader = Some(Loader::WikiProvider(config));
                }
                HUGGING => {
                    let value = map.next_value::<serde_yaml::Value>().unwrap();
                    let config = serde_yaml::from_value::<HuggingConfig>(value).unwrap();
                    let result = arrow_provider::download_huggingface_dataset(config.dataset, 
                        config.arg, 
                        config.key);
                        let location = result.unwrap()[0].to_owned();
                    loader = Some(Loader::ArrowProvider(ArrowConfig{location:location}));
                }
                _ => {}
            }
            
        }
            
        Ok(Source{iterations:iterations.unwrap(), loader:loader.unwrap()})
    }
}

