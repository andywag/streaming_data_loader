use crate::{config::TrainingConfig, tokenizer::tokenizer_config::{TokenizerTask, TokenizerInternalConfig, TokenizerType}, batcher::BatchConfig, datasets::{dataset::DataSet, dataset_config::DataSetConfig}, transport::{zmq_receive::NodeConfig}, provider::{provider_config::{ProviderConfig, ProviderLength, SourceDescription}, pile_datasets::PileDatasetType, source_filter::SourceFilter}, tasks::arrow_cases};

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
    let batch_size = if test {1} else {8192};
    let sequence_length = 512;
    let mask_length = get_mask_length(sequence_length);
    let context_shape = vec![3,3,3,3];

    let batch_config = BatchConfig{batch_size, sequence_length};
    let config = PythonConfig{ mask_length, context_shape:context_shape.clone()};

    let dataset_config = DataSetConfig::Python { mask_length, context_shape };
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
                dataset: DataSet::Python(data),
                dataset_config
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
                dataset_config
            }
        }
    
    }
}
    
