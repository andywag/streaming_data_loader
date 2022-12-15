use crate::{config::{TrainingConfig}, tokenizer::tokenizer_config::{TokenizerTask, TokenizerInternalConfig, TokenizerType}, batcher::BatchConfig, datasets::{dataset_config::DataSetConfig}, transport::{zmq_receive::NodeConfig}, provider::{provider_config::{ProviderConfig, ProviderLength, SourceDescription}, pile_datasets::PileDatasetType, source_filter::SourceFilter}, tasks::arrow_cases};


pub enum Cases {
    Basic, 
    Context,
    Span,
}

fn get_provider(test:bool) -> ProviderConfig {
    let length = if test {
        ProviderLength::Iterations { iterations: 4} 
    }
    else {
        ProviderLength::Epochs { epochs:7 }
    };
    ProviderConfig {
        shuffle: Some(true),
        flatten: Some(true),
        length,
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
    let context_size:usize = 5;//vec![3,3,3,3];

    let batch_config = BatchConfig{batch_size, sequence_length};

    let dataset_config = DataSetConfig::MaskHier { mask_length, context_size, front:false };
    match case {
        Cases::Span => {
            let dataset_config = DataSetConfig::SpanHier { avg_span_prob:0.15, context_size: 5 };
            let tokenizer =  TokenizerInternalConfig{ task:TokenizerTask::T5, typ:TokenizerType::Python};
            TrainingConfig { 
                model_config:crate::config::ModelType::T5,
                source: get_provider(test), 
                tokenizer,
                batch: batch_config, 
                transport: arrow_cases::get_transport_config(test), 
                node: NodeConfig::None, 
                dataset_config
            }
        }
        Cases::Basic => {
            let tokenizer =  TokenizerInternalConfig{ task:TokenizerTask::Bert, typ:TokenizerType::Python};
            TrainingConfig { 
                model_config:crate::config::ModelType::BertHier,
                source: get_provider(test), 
                tokenizer,
                batch: batch_config, 
                transport: arrow_cases::get_transport_config(test), 
                node: NodeConfig::None, 
                dataset_config
            }
        },
        Cases::Context => {
            let tokenizer =  TokenizerInternalConfig{ task:TokenizerTask::Bert, typ:TokenizerType::Python};
            TrainingConfig { 
                model_config:crate::config::ModelType::BertHier,
                source: get_provider(false), 
                tokenizer,
                batch: batch_config, 
                transport: arrow_cases::get_transport_config(test), 
                node: NodeConfig::None, 
                dataset_config
            }
        }
    
    }
}
    
