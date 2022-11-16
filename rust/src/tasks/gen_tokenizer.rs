
use std::collections::VecDeque;

use crate::batcher::Batcher;
use crate::datasets::DataSet;
use crate::tokenizer_wrapper::{TokenizerWrapper};



pub struct GenTokenizer {
    pub batch_size:usize,
    pub sequence_length:usize,
    tokenizer:TokenizerWrapper,
    store:VecDeque<DataSet>, 
    template:DataSet
}
 
impl GenTokenizer {
    pub fn new(_config:&serde_yaml::Value, 
        dataset:DataSet,
        batch_size:usize, 
        sequence_length:usize, 
        tokenizer:TokenizerWrapper) -> Self {
        
        let first_set = dataset.clone().create_data();
        Self {
            batch_size: batch_size,
            sequence_length: sequence_length,
            tokenizer: tokenizer,
            store:VecDeque::from(vec!(first_set)),
            template:dataset
        }
    }


    fn handle_internal_batch(&mut self, ids:&mut [u32]) {
        let result = self.store.back_mut().unwrap().put_data(ids);
        if result {
            self.store.push_back(self.template.create_data());
        }
        
    }
}
    
// TODO : Handle Batch overlap : Currently throws out remaining sequence at the end of the batch
impl Batcher for GenTokenizer {
    type S = String;
    type T = DataSet;

    fn create_sync_batch(&mut self, data:Self::S) -> Option<Self::T> {

       // Tokenize the Data
       let mut ids = self.tokenizer.encode(data);
       // Break the tokenized data into chunks
       let chunks = ids.chunks_mut(self.sequence_length as usize);
       // Encode and Batch the Data
       chunks.into_iter().for_each(|e|self.handle_internal_batch(e));
       // Return the data if it is complete
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


