
use crate::batcher::Batcher;
use crate::tokenizer::tokenizer_wrapper::{TokenizerWrapper};


use super::squad_data::{SquadData, SquadGeneral};


pub struct SquadTokenizer {
    tokenizer:TokenizerWrapper,
    batch:SquadData
}

impl SquadTokenizer {
    pub fn new(batch:SquadData, tokenizer:TokenizerWrapper) -> Self {
        Self {
            tokenizer: tokenizer,
            batch:batch,
        }
    }
}

    impl Batcher for SquadTokenizer {
        type S = SquadGeneral;
        type T = SquadData;

        fn create_sync_batch(&mut self, data:SquadGeneral) -> Option<SquadData> {
            
            //let result = self.tokenizer.get_tokenizer().encode((data.question.clone(), data.context.clone()), true).unwrap();
            let input_data = tokenizers::EncodeInput::Dual(data.question.clone().into(), data.context.clone().into());
            let result = self.tokenizer.encode(input_data);
            let new_result = self.batch.put_data(&result, data);

            if new_result {
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



