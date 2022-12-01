
use crate::batcher::Batcher;

use crate::tokenizer::tokenizer_wrapper::{TokenizerWrapper};

use super::multi_data::{MultiData, MultiTransport};


pub struct MultiTokenizer {
    
    tokenizer:TokenizerWrapper,
    batch:MultiData,
}

impl MultiTokenizer {
    pub fn new(batch:MultiData, tokenizer:TokenizerWrapper) -> Self {
        Self {
            tokenizer: tokenizer,
            batch:batch,
        }
    }

}

    impl Batcher for MultiTokenizer {
        type S = MultiTransport;
        type T = MultiData;

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

