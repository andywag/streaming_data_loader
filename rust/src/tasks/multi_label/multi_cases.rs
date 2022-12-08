use crate::{config::TrainingConfig, tokenizer::tokenizer_config::{TokenizerTask, TokenizerInternalConfig, TokenizerType}, batcher::BatchConfig, datasets::{dataset::DataSet, dataset_config::DataSetConfig}, tasks::arrow_cases, provider::provider_config::{ProviderConfig, HuggingDescription}, transport::zmq_receive::NodeConfig};

use super::{MultiConfig, multi_data::MultiData};


pub fn get_provider(test:bool) -> ProviderConfig {

    let source = HuggingDescription::new("xed_en_fi",Some("en_annotated"),vec!["train"]);
    arrow_cases::get_provider(source, test)
    
}


pub fn get_case(test:bool) -> TrainingConfig {
    let mask_config = MultiConfig{ number_labels: 9 };
    let model = crate::config::TaskType::MultiLabel;
    let tokenizer = TokenizerInternalConfig{ task:TokenizerTask::Bert, 
        typ:TokenizerType::HuggingFace("bert-base-uncased".to_string()) };

    let (batch, dataset) = if test {
        let batch_config = BatchConfig{batch_size:1,sequence_length:128};
        let data = MultiData::new(&mask_config, batch_config.clone());
        (batch_config, data)
    }
    else {
        let batch_config = BatchConfig{batch_size:2048,sequence_length:128};
        let data = MultiData::new(&mask_config, batch_config.clone());
        (batch_config, data)
    };
            
    TrainingConfig { 
        model, 
        source: get_provider(test), 
        tokenizer,
        batch, 
        transport:arrow_cases::get_transport_config(test), 
        node: NodeConfig::None, 
        dataset:DataSet::Multi(dataset),
        dataset_config: DataSetConfig::MultiLabel{number_labels: 9}
    }
        
}
    
