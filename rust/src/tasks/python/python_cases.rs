use crate::{config::TrainingConfig, tokenizer::tokenizer_config::{TokenizerTask, TokenizerInternalConfig, TokenizerType}, batcher::BatchConfig, datasets::DataSet, transport::{zmq_receive::NodeConfig}, provider::{provider_config::{ProviderConfig, ProviderLength, SourceDescription}, pile_datasets::PileDatasetType, source_filter::SourceFilter}, tasks::arrow_cases};

use super::{config::PythonConfig, python_data::PythonData};

pub enum Cases {
    Basic, 
    Context
}

pub fn get_provider() -> ProviderConfig {
    ProviderConfig {
        shuffle: Some(true),
        flatten: Some(true),
        length: ProviderLength::Epochs { epochs:7 },
        source: SourceDescription::Pile { typ:PileDatasetType::GithubDataset },
        filter: Some(SourceFilter::PythonText),
    } 
}

fn get_mask_length(sequence_length:usize) -> usize {
    (sequence_length as f32 * 0.15) as usize
}

pub fn get_case(case:Cases, test:bool) -> TrainingConfig {
    let b = if test {1} else {8192};

    let batch_config = BatchConfig{batch_size:b,sequence_length:512};
    let config = PythonConfig{ mask_length: get_mask_length(batch_config.sequence_length),
        context_shape: vec![2,2,4,4]
    };
    match case {
        Cases::Basic => {
            let data = PythonData::new(config, batch_config.clone(), 5);
            let tokenizer =  TokenizerInternalConfig{ task:TokenizerTask::Bert, typ:TokenizerType::Python};
            TrainingConfig { 
                model: crate::config::TaskType::Python, 
                source: get_provider(), 
                tokenizer,
                batch: batch_config, 
                transport: arrow_cases::get_transport_config(test), 
                node: NodeConfig::None, 
                dataset: DataSet::Python(data) 
            }
        },
        Cases::Context => {
            let data = PythonData::new(config, batch_config.clone(), 5);
            let tokenizer =  TokenizerInternalConfig{ task:TokenizerTask::Bert, typ:TokenizerType::Python};
            TrainingConfig { 
                model: crate::config::TaskType::Context, 
                source: get_provider(), 
                tokenizer,
                batch: batch_config, 
                transport: arrow_cases::get_transport_config(test), 
                node: NodeConfig::None, 
                dataset: DataSet::Python(data),
            }
        }
    
    }
}
    
