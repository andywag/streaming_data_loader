use crate::{datasets::{dataset_config::DataSetConfig, dataset::DataSet}, batcher::{BatchConfig, Batcher}, tokenizer::tokenizer_wrapper::TokenizerWrapper, config::ModelType};

use super::{simple_transport::SimpleTransport};




pub struct SimpleBatcher {
    model_type:ModelType,
    dataset_config:DataSetConfig,
    batch_config:BatchConfig,
    tokenizer:TokenizerWrapper,

    batch:DataSet
}

impl SimpleBatcher {
    pub fn new(model_type:ModelType, dataset_config:DataSetConfig,  batch_config:BatchConfig, tokenizer:TokenizerWrapper) -> Self {
        Self {
            batch: model_type.create_dataset(dataset_config.clone(), batch_config.clone(), tokenizer.get_tokenizer_info()),
            model_type,
            dataset_config,
            batch_config,
            tokenizer,

            
        }
    }
}

impl Batcher for SimpleBatcher {
    type S = SimpleTransport;
    type T = DataSet;

    fn create_sync_batch(&mut self, data:Self::S) -> Option<Self::T> {
        let result = self.tokenizer.encode_simple(data.data);
        let result = self.batch.put_full_data(result.0, result.1, data.label);

        if result {
            return self.get_working_batch();
        }
        return None;
    }

    fn get_working_batch(&mut self) -> Option<Self::T> {
        
        let mut old_batch = self.model_type.create_dataset(self.dataset_config.clone(), 
            self.batch_config.clone(),
            self.tokenizer.get_tokenizer_info()); 
        std::mem::swap(&mut self.batch, &mut old_batch);
        return Some(old_batch);
    }

}

