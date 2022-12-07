use crate::{config::TrainingConfig, tokenizer::tokenizer_config::{TokenizerTask, TokenizerInternalConfig, TokenizerType}, batcher::BatchConfig, datasets::DataSet, transport::{zmq_receive::NodeConfig}, provider::{provider_config::{ProviderConfig,HuggingDescription}}, tasks::arrow_cases};
use super::{ SingleClassConfig, single_data::SingleClassData};


pub enum Cases {
    Imdb, 
}

pub fn get_provider(test:bool) -> ProviderConfig {

    let source = HuggingDescription::new("imdb",None,vec!["train"]);
    arrow_cases::get_provider(source, test)

}


pub fn get_case(typ:Cases, test:bool) -> TrainingConfig {
    let mask_config = SingleClassConfig{};
    let model = crate::config::TaskType::SingleClass;
    let tokenizer = TokenizerInternalConfig{ task:TokenizerTask::Bert, 
        typ:TokenizerType::HuggingFace("bert-base-uncased".to_string()) };

    match typ {
        Cases::Imdb => {
            let (batch, dataset) = if test {
                let batch_config = BatchConfig{batch_size:1,sequence_length:128};
                let data = SingleClassData::new(&mask_config, batch_config.clone());
                (batch_config, data)
            }
            else {
                let batch_config = BatchConfig{batch_size:2048,sequence_length:128};
                let data = SingleClassData::new(&mask_config, batch_config.clone());
                (batch_config, data)
            };
            

            TrainingConfig { 
                model, 
                source: get_provider(test), 
                tokenizer,
                batch, 
                transport:arrow_cases::get_transport_config(test), 
                node: NodeConfig::None, 
                dataset:DataSet::Single(dataset)
            }
        }
        
    }
    
}