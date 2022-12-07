use std::collections::VecDeque;

use crate::batcher::{Batcher, BatchConfig};
use crate::datasets::DataSet;


use super::python_runner::PythonTokenizer;



pub struct PythonBatch {
    dataset:DataSet,
    _batch_config:BatchConfig,
    tokenizer:PythonTokenizer,
    store:VecDeque<DataSet>
}
 
impl PythonBatch {
    pub fn new(dataset:DataSet,
        batch_config:BatchConfig, 
        tokenizer:PythonTokenizer,
    ) -> Self {
        
        let first_set = dataset.clone().create_data();//PythonData::new(config, batch_config, 5);
        Self {
            dataset:dataset,
            _batch_config:batch_config,
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
                self.store.push_back(self.dataset.create_data());
            },
            None => {
                //log::info!("Adding Data to Queue");
                self.store.push_back(self.dataset.create_data());
            }
        }
        

        if ids.is_none() {
            return None;
        }
        match self.store.back_mut().unwrap() {
            DataSet::Python(x) => x.put_data(ids.unwrap()),
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


