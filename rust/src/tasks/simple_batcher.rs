
use crate::batcher::{Batcher, BatchConfig};

use crate::datasets::dataset::DataSet;
use crate::datasets::dataset_config::DataSetConfig;
use crate::tokenizer::tokenizer_wrapper::{TokenizerWrapper};



pub struct MultiTokenizer {
    dataset_config:DataSetConfig,
    batch_config:BatchConfig,
    tokenizer:TokenizerWrapper,
}

impl MultiTokenizer {
    pub fn new(dataset_config:DataSetConfig, batch_config:BatchConfig, tokenizer:TokenizerWrapper) -> Self {
        Self {
            dataset_config,
            batch_config,
            tokenizer
        }
    }

}

    impl Batcher for MultiTokenizer {
        type S = MultiTransport;
        type T = DataSet;

        fn create_sync_batch(&mut self, data:Self::S) -> Option<Self::T> {
            //log::info!("Data {:?}", data);
            let result = self.tokenizer.encode(data.text.into());
            let result = self.batch.put_data(&result, data.labels);
            if result {
                return self.get_working_batch();
            }
            return None;
            
    }

    fn get_working_batch(&mut self) -> Option<Self::T> {
        let mut old_batch = self.batch.new_data(); 
        std::mem::swap(&mut self.batch, &mut old_batch);

        return Some(old_batch);
    }

}

