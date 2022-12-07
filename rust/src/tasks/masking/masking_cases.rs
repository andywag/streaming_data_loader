use crate::{config::TrainingConfig, tokenizer::tokenizer_config::{TokenizerTask, TokenizerInternalConfig, TokenizerType}, batcher::BatchConfig, datasets::DataSet, transport::{zmq_receive::NodeConfig}, provider::{provider_config::{ProviderConfig, ProviderLength, SourceDescription, Dataset}, pile_datasets::PileDatasetType}, tasks::arrow_cases};
use super::{masked_data::MaskedData, gpt_data::GptData, t5_data::T5Data};

pub enum MaskingCases {
    Bert, 
    //Roberta,
    Gpt,
    T5
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
            source: SourceDescription::Pile { typ:PileDatasetType::Wiki },
            filter: None,
        }
    }
}

fn get_mask_length(sequence_length:usize) -> usize {
    (sequence_length as f32 * 0.15) as usize
}

pub fn get_case(typ:MaskingCases, test:bool) -> TrainingConfig {
    let batch = if test {
        BatchConfig{ batch_size: 1, sequence_length: 128}
    }
    else {
        BatchConfig{ batch_size: 4096, sequence_length: 128}
    };

    match typ {
        MaskingCases::Bert => {
            
            let mask_config = super::MaskingConfig { mask_length: get_mask_length(batch.sequence_length) };
            let data = MaskedData::new(mask_config, batch.clone(), 103);
            let tokenizer = TokenizerInternalConfig{ task:TokenizerTask::Bert, 
                typ:TokenizerType::HuggingFace("bert-base-uncased".to_string()) 
            }; 
            TrainingConfig { 
                model: crate::config::TaskType::Mlm, 
                source: get_provider(true), 
                tokenizer,
                batch, 
                transport: arrow_cases::get_transport_config(test), 
                node: NodeConfig::None, 
                dataset: DataSet::Mask(data) 
            }
        },
        MaskingCases::Gpt => {            
            let data = GptData::new(batch.clone());
            let tokenizer = TokenizerInternalConfig{
                task:TokenizerTask::Gpt, 
                typ:TokenizerType::HuggingFace("gpt2".to_string()) 
            }; 
            TrainingConfig { 
                model: crate::config::TaskType::Causal, 
                source: get_provider(true), 
                tokenizer,
                batch, 
                transport: arrow_cases::get_transport_config(test), 
                node: NodeConfig::None, 
                dataset: DataSet::Gpt2(data) 
            }
        },
        MaskingCases::T5 => {
            
            let mask_config = super::T5Config { number_spans: batch.sequence_length/8, mask_probability: 0.15 };
            let data = T5Data::new(mask_config, batch.clone(), vec![0;100]);
            let tokenizer = TokenizerInternalConfig{ task:TokenizerTask::T5, 
                typ:TokenizerType::HuggingFace("t5-small".to_string()) 
            }; 
            TrainingConfig { 
                model: crate::config::TaskType::T5, 
                source: get_provider(true), 
                tokenizer,
                batch, 
                transport: arrow_cases::get_transport_config(test), 
                node: NodeConfig::None, 
                dataset: DataSet::T5(data) 
            }
        },
    }
    
}