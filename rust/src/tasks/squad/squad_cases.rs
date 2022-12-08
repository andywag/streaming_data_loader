use crate::{config::TrainingConfig, tokenizer::tokenizer_config::{TokenizerTask, TokenizerInternalConfig, TokenizerType}, batcher::BatchConfig, datasets::{dataset::DataSet, dataset_config::DataSetConfig}, tasks::arrow_cases, provider::provider_config::{ProviderConfig, HuggingDescription}, transport::zmq_receive::NodeConfig};

use super::{squad_data::SquadData};



pub fn get_provider(test:bool) -> ProviderConfig {

    let source = HuggingDescription::new("squad",None,vec![""]);
    arrow_cases::get_provider(source, test)

}


pub fn get_case(test:bool) -> TrainingConfig {
    let model = crate::config::TaskType::Squad;
    let tokenizer = TokenizerInternalConfig{ task:TokenizerTask::Bert, 
        typ:TokenizerType::HuggingFace("bert-base-uncased".to_string()) };

    let (batch, dataset) = if test {
        let batch_config = BatchConfig{batch_size:1,sequence_length:128};
        let data = SquadData::new(batch_config.clone());
        (batch_config, data)
    }
    else {
        let batch_config = BatchConfig{batch_size:2048,sequence_length:128};
        let data = SquadData::new(batch_config.clone());
        (batch_config, data)
    };
            
    TrainingConfig { 
        model, 
        source: get_provider(test), 
        tokenizer,
        batch, 
        transport:arrow_cases::get_transport_config(test), 
        node: NodeConfig::None, 
        dataset:DataSet::Squad(dataset),
        dataset_config:DataSetConfig::Squad
    }
        
}
    
