use crate::{batcher::Batcher, tokenizer::tokenizer_wrapper::{TokenizerWrapper}};


use super::{single_data::{SingleClassData, SingleClassTransport}};



pub struct SingleTokenizer {
    tokenizer:TokenizerWrapper,
    batch:SingleClassData
}

impl SingleTokenizer {
    pub fn new(batch:SingleClassData, tokenizer:TokenizerWrapper) -> Self {
        Self {
            tokenizer: tokenizer,
            batch:batch,
        }
    }

}

    impl Batcher for SingleTokenizer {
        type S = SingleClassTransport;
        type T = SingleClassData;

        fn create_sync_batch(&mut self, data:Self::S) -> Option<Self::T> {
            let result = self.tokenizer.encode(data.text.into());
            let result = self.batch.put_data(&result, data.label);

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

