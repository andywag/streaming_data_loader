use clap::ValueEnum;
use serde::{Deserialize, Serialize};

use crate::datasets::dataset::DataSet;
use crate::datasets::dataset_config::DataSetConfig;
use crate::models::bert_data::BertData;
use crate::models::gpt_data::GptData;
use crate::models::hier_bert_data::BertHierData;
use crate::models::t5_data::T5Data;
use crate::provider::provider_config::ProviderConfig;
use crate::tokenizer::tokenizer_config::{TokenizerInternalConfig};
use crate::batcher::BatchConfig;
use crate::tokenizer::tokenizer_wrapper::{TokenizerInfo};
use crate::transport::TransportConfig;
use crate::transport::zmq_receive::NodeConfig;



#[derive(Deserialize, Serialize, Debug, Clone, ValueEnum)]
pub enum TaskType {
    Mlm,
    Clm,
    Squad,
    MultiLabel,
    SingleClass,
    Span,
    Python,
    Context,
    SpanPython
}

#[derive(Deserialize, Serialize, Debug, Clone, ValueEnum)]

pub enum ModelType {
    Bert,
    Roberta,
    Gpt2,
    T5,
    BertHier
}

impl ModelType {
    pub fn create_dataset(&self, dataset_config:DataSetConfig, batch_config:BatchConfig, tokenizer_info:TokenizerInfo) -> DataSet{
        match self {
            ModelType::Bert =>  {
                BertData::new(batch_config, dataset_config).into()
            }
            ModelType::Gpt2 =>  {
                GptData::new(batch_config, dataset_config).into()
            }
            ModelType::T5 =>  {
                T5Data::new(batch_config, dataset_config, tokenizer_info).into()
            }
            ModelType::BertHier => {
                BertHierData::new(batch_config, dataset_config, 5).into()
            }
            _ => todo!()
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]

pub struct TrainingConfig {
    pub model_config:ModelType,
    pub source:ProviderConfig,
    pub tokenizer:TokenizerInternalConfig,
    pub batch:BatchConfig,
    pub transport:TransportConfig,
    pub node:NodeConfig,
    pub dataset_config:DataSetConfig
}