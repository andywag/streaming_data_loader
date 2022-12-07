use std::vec;

use crate::batcher::BatchConfig;
use crate::config::{TrainingConfig, TaskType};
use crate::datasets::DataSet;
use crate::provider::provider_config::{self, ProviderConfig};
use crate::tokenizer::tokenizer_config::{TokenizerTask, TokenizerType, TokenizerInternalConfig};
use crate::transport::{TransportConfig, TransportEnum};
use crate::transport::zmq_receive::NodeConfig;

use super::masking::masking_cases::MaskingCases;
use super::masking::t5_data::T5Data;
use super::masking::{MaskingConfig, T5Config, masking_cases};
use super::masking::gpt_data::GptData;
use super::masking::masked_data::MaskedData;
use super::multi_label::multi_cases;
use super::python::python_cases;
use super::single_class::single_cases;
use super::squad::squad_cases;




fn get_basic_batch_config() -> BatchConfig {
    BatchConfig{ batch_size: 16, sequence_length: 128 }
}

fn get_basic_config(model:TaskType,
        provider_config:ProviderConfig,
        task:TokenizerTask, 
        typ:TokenizerType,
        batch_config:BatchConfig, 
        dataset:DataSet,
        transport_config:TransportConfig,
    ) -> TrainingConfig {
    TrainingConfig {
        model: model,
        source: provider_config,
        tokenizer: TokenizerInternalConfig{ task:task, typ:typ },
        batch: batch_config,
        transport: transport_config,
        node: NodeConfig::None,
        dataset:dataset
    }
}

pub enum BasicCases {
    Bert, 
    Roberta,
    Gpt,
    T5,
    Squad,
    Multi,
    Single,
    Python,
    PythonNew,
    PythonContext
}

impl BasicCases {
    pub fn get_config(&self, test:bool) -> TrainingConfig {
        let transport_config = if test {
            TransportConfig{ transport: TransportEnum::Test }
        }
        else {
            TransportConfig{ transport: TransportEnum::Zmq { address: "ipc:///tmp/masking_train".to_string()} }
        };
        let batch_config = if test {
            get_basic_batch_config()
        }
        else {
            BatchConfig{ batch_size: 32768, sequence_length: 128 }
        };

        match self {
            BasicCases::Bert => {
                if test {
                    masking_cases::get_case(MaskingCases::Test) 
                }
                else {
                    masking_cases::get_case(MaskingCases::Basic)
                }
            },
            BasicCases::Roberta => {
                let mask_config = MaskingConfig{ mask_length: (0.15*(batch_config.sequence_length as f32)) as usize };
                let data = MaskedData::new(mask_config, batch_config.clone(), 5); // TODO : Not sure of token also fix hard code 

                get_basic_config(crate::config::TaskType::Mlm,
                    provider_config::Examples::Mask.get_config(test),
                    TokenizerTask::Roberta, 
                    TokenizerType::HuggingFace("roberta-base".to_string()),
                    batch_config,
                    DataSet::Mask(data),
                    transport_config)
            },
            BasicCases::Gpt => {
                let data = GptData::new(batch_config.clone()); // TODO : Not sure of token also fix hard code 

                get_basic_config(crate::config::TaskType::Causal,
                    provider_config::Examples::Mask.get_config(test),
                    TokenizerTask::Gpt, 
                    TokenizerType::HuggingFace("gpt2".to_string()),
                    batch_config,
                    DataSet::Gpt2(data),
                    transport_config)
            },
            BasicCases::T5 => {
                let config = T5Config{ number_spans: batch_config.sequence_length/8, mask_probability: 0.15 };
                // TODO : Fix the extra data for t5
                let data = T5Data::new(config, batch_config.clone(), vec![0;100]); 
                get_basic_config(crate::config::TaskType::T5,
                    provider_config::Examples::Mask.get_config(test),
                    TokenizerTask::T5, 
                    TokenizerType::HuggingFace("t5-small".to_string()),
                    batch_config,
                    DataSet::T5(data),
                    transport_config)
            },
            BasicCases::Squad => squad_cases::get_case(test),
            BasicCases::Multi => multi_cases::get_case(test),
            BasicCases::Single => single_cases::get_case(single_cases::Cases::Imdb, test),
            /* 
            BasicCases::Python => {
                let batch_config = if test {BatchConfig{batch_size:1,sequence_length:128}} else {BatchConfig{batch_size:32768,sequence_length:512}};
                let batch_config = batch_config;
                let mask_config = MaskingConfig{ mask_length: (0.15*(batch_config.sequence_length as f32)) as usize };
                let data = MaskedData::new(mask_config, batch_config.clone(), 5);

                get_basic_config(crate::config::TaskType::Python,
                    provider_config::Examples::Python.get_config(test),
                    TokenizerTask::Bert, 
                    TokenizerType::Python,
                    batch_config,
                    DataSet::Mask(data),
                    transport_config)
            },*/
            BasicCases::Python | BasicCases::PythonNew => python_cases::get_case(test),
            BasicCases::PythonContext => {
                let batch_config = BatchConfig { batch_size: 8192, sequence_length: 512};
                let mask_config = MaskingConfig{ mask_length: (0.15*(batch_config.sequence_length as f32)) as usize };
                let data = MaskedData::new(mask_config, batch_config.clone(), 5);

                get_basic_config(crate::config::TaskType::Mlm,
                    provider_config::Examples::Python.get_config(test),
                    TokenizerTask::Bert, 
                    TokenizerType::PythonContext,
                    batch_config,
                    DataSet::Mask(data),
                    TransportConfig{ transport: TransportEnum::Test })
            },

        }
    }
}