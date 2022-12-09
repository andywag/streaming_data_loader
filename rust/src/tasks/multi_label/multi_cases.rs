use crate::{config::TrainingConfig, tokenizer::tokenizer_config::{TokenizerTask, TokenizerInternalConfig, TokenizerType}, batcher::BatchConfig, datasets::{dataset_config::DataSetConfig}, tasks::arrow_cases, provider::provider_config::{ProviderConfig, HuggingDescription}, transport::zmq_receive::NodeConfig};



pub fn get_provider(test:bool) -> ProviderConfig {

    let source = HuggingDescription::new("xed_en_fi",Some("en_annotated"),vec!["train"]);
    arrow_cases::get_provider(source, test)
    
}


pub fn get_case(test:bool) -> TrainingConfig {
    let tokenizer = TokenizerInternalConfig{ task:TokenizerTask::Bert, 
        typ:TokenizerType::HuggingFace("bert-base-uncased".to_string()) };

    let batch = if test {
        let batch_config = BatchConfig{batch_size:1,sequence_length:128};
        batch_config
    }
    else {
        let batch_config = BatchConfig{batch_size:2048,sequence_length:128};
        batch_config
    };
            
    TrainingConfig { 
        model_config:crate::config::ModelType::Bert,
        source: get_provider(test), 
        tokenizer,
        batch, 
        transport:arrow_cases::get_transport_config(test), 
        node: NodeConfig::None, 
        dataset_config: DataSetConfig::MultiLabel{number_labels: 9}
    }
        
}
    
