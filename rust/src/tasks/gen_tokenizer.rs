
use std::collections::VecDeque;

use crate::batcher::Batcher;
use crate::datasets::DataSet;
use crate::tokenizer::tokenizer_wrapper::{TokenizerWrapper};



pub struct GenTokenizer {
    pub batch_size:usize,
    pub sequence_length:usize,
    tokenizer:TokenizerWrapper,
    store:VecDeque<DataSet>, 
    template:DataSet, 
    chunk:bool
}
 
impl GenTokenizer {
    pub fn new(dataset:DataSet,
        batch_size:usize, 
        sequence_length:usize, 
        tokenizer:TokenizerWrapper,
        chunk:bool
    ) -> Self {
        
        let first_set = dataset.clone().create_data();
        Self {
            batch_size: batch_size,
            sequence_length: sequence_length,
            tokenizer: tokenizer,
            store:VecDeque::from(vec!(first_set)),
            template:dataset, 
            chunk:chunk
        }
    }


    fn handle_internal_batch(&mut self, ids:&mut [u32]) {
        let _result = self.store.back_mut().unwrap().put_data(ids);
        //log::info!("Here {} {}", result, self.store.back().unwrap().done());
        if self.store.back().unwrap().done() {
            let remaining = self.store.back().unwrap().remaining();
            self.store.push_back(self.template.create_data());
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
        // Break the tokenized data into chunks
        if self.chunk {
            let chunks = ids.chunks_mut(self.sequence_length as usize);
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


