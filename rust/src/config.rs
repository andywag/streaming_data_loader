use serde::{Deserialize, Serialize};

use crate::datasets::DataSet;
use crate::provider::provider_config::ProviderConfig;
use crate::tokenizer::tokenizer_config::{TokenizerInternalConfig};
use crate::batcher::BatchConfig;
use crate::transport::TransportConfig;
use crate::transport::zmq_receive::NodeConfig;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum TaskType {
    #[serde(rename="masking")]
    Mlm,
    #[serde(rename="causal")]
    Causal,
    #[serde(rename="squad")]
    Squad,
    #[serde(rename="multi-label")]
    MultiLabel,
    #[serde(rename="single-class")]
    SingleClass,
    #[serde(rename="t5")]
    T5,
    Python,
}

#[derive(Deserialize, Serialize, Debug, Clone)]

pub struct TrainingConfig {
    pub model:TaskType,
    pub source:ProviderConfig,
    pub tokenizer:TokenizerInternalConfig,
    pub batch:BatchConfig,
    pub transport:TransportConfig,
    pub node:NodeConfig,
    pub dataset:DataSet
}