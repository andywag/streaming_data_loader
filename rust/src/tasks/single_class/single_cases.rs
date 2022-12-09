use crate::{config::{TrainingConfig, ModelType}, tokenizer::tokenizer_config::{TokenizerTask, TokenizerInternalConfig, TokenizerType}, batcher::BatchConfig, datasets::{dataset_config::DataSetConfig}, transport::{zmq_receive::NodeConfig}, provider::{provider_config::{ProviderConfig,HuggingDescription}}, tasks::arrow_cases};


pub enum Cases {
    Imdb, 
}

pub fn get_provider(test:bool) -> ProviderConfig {

    let source = HuggingDescription::new("imdb",None,vec!["train"]);
    arrow_cases::get_provider(source, test)

}


pub fn get_case(typ:Cases, test:bool) -> TrainingConfig {
    let tokenizer = TokenizerInternalConfig{ task:TokenizerTask::Bert, 
        typ:TokenizerType::HuggingFace("bert-base-uncased".to_string()) };

    match typ {
        Cases::Imdb => {
            let batch = if test {
                let batch_config = BatchConfig{batch_size:1,sequence_length:128};
                batch_config
            }
            else {
                let batch_config = BatchConfig{batch_size:2048,sequence_length:128};
                batch_config
            };
            

            TrainingConfig { 
                model_config:ModelType::Bert,
                source: get_provider(test), 
                tokenizer,
                batch, 
                transport:arrow_cases::get_transport_config(test), 
                node: NodeConfig::None, 
                dataset_config:DataSetConfig::SingleClass
            }
        }
        
    }
    
}