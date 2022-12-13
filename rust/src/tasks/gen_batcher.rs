
use std::collections::VecDeque;

use crate::batcher::{Batcher, BatchConfig};
use crate::config::ModelType;
use crate::datasets::dataset::DataSet;
use crate::datasets::dataset_config::DataSetConfig;
use crate::tokenizer::tokenizer_wrapper::{TokenizerWrapper};



pub struct GenTokenizer {
    model_type:ModelType,
    batch_config:BatchConfig,
    dataset_config:DataSetConfig,
    tokenizer:TokenizerWrapper,
    store:VecDeque<DataSet>, 
    //template:DataSet, 
    chunk:bool
}
 
impl GenTokenizer {
    pub fn new(
        model_type:ModelType,
        batch_config:BatchConfig, 
        dataset_config:DataSetConfig,
        tokenizer:TokenizerWrapper,
        chunk:bool
    ) -> Self {
        
        let first_set = model_type.create_dataset(dataset_config.clone(), batch_config.clone(), tokenizer.get_tokenizer_info());
        Self {
            model_type,
            batch_config,
            dataset_config,
            tokenizer: tokenizer,
            store:VecDeque::from(vec!(first_set)),
            //template:dataset, 
            chunk:chunk
        }
    }


    fn handle_internal_batch(&mut self, ids:&mut [u32]) {
        let _result = self.store.back_mut().unwrap().put_full_data(ids.to_vec(), None, None);
        //log::info!("Here {} {}", result, self.store.back().unwrap().done());
        if self.store.back().unwrap().done() {
            let remaining = self.store.back().unwrap().remaining();
            //self.store.push_back(self.template.create_data());
            let new_data = self.model_type.create_dataset(self.dataset_config.clone(), 
                self.batch_config.clone(),
                self.tokenizer.get_tokenizer_info()
            );
            self.store.push_back(new_data);
            if remaining.is_some() {
                let mut r = remaining.unwrap();
                let l = r.len();
                self.handle_internal_batch(&mut r[0..l]);
            }
        }
        
    }
}
    
impl Batcher for GenTokenizer {
    type S = String;
    type T = DataSet;

    fn create_sync_batch(&mut self, data:Self::S) -> Option<Self::T> {
        // Tokenize the Data    
        let mut ids = self.tokenizer.encode_mask(data);
        
        // Don't create the data if there isn't enough data
        if ids.len() < 64 {
            return None;
        }
        // Break the tokenized data into chunks
        if self.chunk {
            let chunks = ids.chunks_mut(self.batch_config.sequence_length as usize);
            chunks.into_iter().for_each(|e|self.handle_internal_batch(e));
        }
        else {
            let l = ids.len();
            self.handle_internal_batch(&mut ids[0..l]);
        }
       if self.store.front().unwrap().done() {
            self.store.pop_front()
       }
       else {
            None
       }
        
        
    }

    fn get_working_batch(&mut self) -> Option<Self::T> {
        return self.store.pop_front()
    }

}


