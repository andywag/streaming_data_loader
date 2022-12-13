use crate::{config::TrainingConfig, tokenizer::tokenizer_config::{TokenizerTask, TokenizerInternalConfig, TokenizerType}, batcher::BatchConfig, datasets::{dataset_config::DataSetConfig}, transport::{zmq_receive::NodeConfig}, provider::{provider_config::{ProviderConfig, ProviderLength, SourceDescription, Dataset}}, tasks::arrow_cases};

pub enum MaskingCases {
    Bert, 
    //Roberta,
    Gpt,
    T5
}

//https://dumps.wikimedia.org/other/cirrussearch/current/enwiki-20221021-cirrussearch-content.json.gz

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
            source: SourceDescription::DataList(vec!["https://dumps.wikimedia.org/other/cirrussearch/20221021/enwiki-20221021-cirrussearch-content.json.gz".into()]),
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
            let mask_length = get_mask_length(batch.sequence_length);
            let tokenizer = TokenizerInternalConfig{ 
                task:TokenizerTask::Bert, 
                typ:TokenizerType::HuggingFace("bert-base-uncased".to_string()) 
            }; 
            TrainingConfig { 
                model_config:crate::config::ModelType::Bert,
                source: get_provider(test), 
                tokenizer,
                batch, 
                transport: arrow_cases::get_transport_config(test), 
                node: NodeConfig::None, 
                dataset_config: DataSetConfig::Mask { mask_length , mask: 103}
            }
        },
        MaskingCases::Gpt => {            
            let tokenizer = TokenizerInternalConfig{
                task:TokenizerTask::Gpt, 
                typ:TokenizerType::HuggingFace("gpt2".to_string()) 
            }; 
            TrainingConfig { 
                model_config:crate::config::ModelType::Gpt2,
                source: get_provider(test), 
                tokenizer,
                batch, 
                transport: arrow_cases::get_transport_config(test), 
                node: NodeConfig::None, 
                dataset_config: DataSetConfig::Gpt 
            }
        },
        MaskingCases::T5 => {
            let tokenizer = TokenizerInternalConfig{ task:TokenizerTask::T5, 
                typ:TokenizerType::HuggingFace("t5-small".to_string()) 
            }; 
            TrainingConfig { 
                model_config:crate::config::ModelType::T5,
                source: get_provider(test), 
                tokenizer,
                batch, 
                transport: arrow_cases::get_transport_config(test), 
                node: NodeConfig::None, 
                dataset_config:DataSetConfig::Span{ avg_span_gap: 16.0, avg_span_size: 2.0 } 
            }
        },
    }
    
}