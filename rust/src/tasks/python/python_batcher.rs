use std::collections::VecDeque;

use crate::batcher::{Batcher, BatchConfig};
use crate::config::ModelType;
use crate::datasets::dataset::DataSet;
use crate::datasets::dataset_config::DataSetConfig;


use super::python_runner::PythonTokenizer;



pub struct PythonBatch {
    model_type:ModelType,
    dataset_config:DataSetConfig,
    batch_config:BatchConfig,
    tokenizer:PythonTokenizer,
    store:VecDeque<DataSet>
}
 
impl PythonBatch {
    pub fn new(model_type:ModelType,
        dataset_config:DataSetConfig,
        batch_config:BatchConfig, 
        tokenizer:PythonTokenizer,
    ) -> Self {
        
        let first_set = model_type.create_dataset(dataset_config.clone(), batch_config.clone(), tokenizer.get_tokenizer_info());
        Self {
            model_type,
            dataset_config,
            batch_config:batch_config,
            tokenizer: tokenizer,
            store:VecDeque::from(vec!(first_set))
        }
    }
}
    
impl Batcher for PythonBatch {
    type S = String;
    type T = DataSet;

    fn create_sync_batch(&mut self, data:Self::S) -> Option<Self::T> {
        // Tokenize the Data    
        let ids = self.tokenizer.encode(data);
        match self.store.back() {
            Some(x) => if x.done() {
                self.store.push_back(self.model_type.create_dataset(self.dataset_config.clone(), 
                self.batch_config.clone(),
                self.tokenizer.get_tokenizer_info()));
            },
            None => {
                self.store.push_back(self.model_type.create_dataset(self.dataset_config.clone(), 
                self.batch_config.clone(),
                self.tokenizer.get_tokenizer_info()));
            }
        }
        

        if ids.is_none() {
            return None;
        }
        match self.store.back_mut().unwrap() {
            DataSet::BertHier(x) => x.put_data(ids.unwrap(), None),
            DataSet::T5(x) => x.put_tokenized_data(ids.unwrap(), None),

            _ => {
                log::error!("Only Python Data Set Supported");
                std::process::exit(1);
            }
        };
        
        // Don't create the data if there isn't enough data
        if self.store.front().unwrap().done() {
            let dataset = self.store.pop_front();
            dataset
        }
        else {
            None
        }
        
        
    }

    fn get_working_batch(&mut self) -> Option<Self::T> {
        return self.store.pop_front()
    }

}


