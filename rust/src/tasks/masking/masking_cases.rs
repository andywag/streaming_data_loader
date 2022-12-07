use crate::{config::TrainingConfig, tokenizer::tokenizer_config::{TokenizerTask, TokenizerInternalConfig, TokenizerType}, batcher::BatchConfig, datasets::DataSet, transport::{TransportConfig, TransportEnum, zmq_receive::NodeConfig}, provider::{provider_config::{ProviderConfig, ProviderLength, SourceDescription, Dataset}, pile_datasets::PileDatasetType}};

use super::masked_data::MaskedData;



pub enum MaskingCases {
    Test, 
    Basic,
}

pub fn get_provider(test:bool) -> ProviderConfig {
    if test {
        ProviderConfig {
            shuffle: None,
            flatten: None,
            length: ProviderLength::Iterations { iterations: 10 },
            source: SourceDescription::DataList(vec![Dataset{location:"../data/test.json.gz".to_string()}]),
            filter: None,
        }
    }
    else {
        ProviderConfig {
            shuffle: None,
            flatten: None,
            length: ProviderLength::Epochs { epochs : 1 },
            source: SourceDescription::Pile { typ:PileDatasetType::Total },
            filter: None,
        }
    }
}

fn get_mask_length(sequence_length:usize) -> usize {
    (sequence_length as f32 * 0.15) as usize
}

pub fn get_case(typ:MaskingCases) -> TrainingConfig {
    
    match typ {
        MaskingCases::Test => {
            let batch_config = BatchConfig{ batch_size: 1, sequence_length: 128};
            let mask_config = super::MaskingConfig { mask_length: get_mask_length(128) };
            let data = MaskedData::new(mask_config, batch_config.clone(), 103);
            TrainingConfig { 
                model: crate::config::TaskType::Mlm, 
                source: get_provider(true), 
                tokenizer: TokenizerInternalConfig{ 
                    task:TokenizerTask::Bert, 
                    typ:TokenizerType::HuggingFace("bert-base-uncased".to_string()) 
                }, 
                batch: BatchConfig{ batch_size: 128, sequence_length: 128 }, 
                transport: TransportConfig{transport:TransportEnum::Test}, 
                node: NodeConfig::None, 
                dataset: DataSet::Mask(data) 
            }
        }
        MaskingCases::Basic =>  {
            let batch_config = BatchConfig{ batch_size: 32768, sequence_length: 128};
            let mask_config = super::MaskingConfig { mask_length: get_mask_length(128) };
            let data = MaskedData::new(mask_config, batch_config.clone(), 103);
            TrainingConfig { 
                model: crate::config::TaskType::Mlm, 
                source: get_provider(false), 
                tokenizer: TokenizerInternalConfig{ 
                    task:TokenizerTask::Bert, 
                    typ:TokenizerType::HuggingFace("bert-base-uncased".to_string()) 
                }, 
                batch: BatchConfig{ batch_size: 8192, sequence_length: 128 }, 
                transport: TransportConfig{transport:TransportEnum::Zmq { address:"ipc:///tmp/masking_train".to_string() }}, 
                node: NodeConfig::None, 
                dataset: DataSet::Mask(data) 
            }
        }
    }
    
}