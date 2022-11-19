
use crate::{batcher::Batcher, tokenizer_wrapper::{TokenizerWrapper, self}, datasets::data_generator::{TextSupplier, DataGenerator}};




pub struct GenericTokenizer<T:DataGenerator> {

    tokenizer:TokenizerWrapper,
    batch:T
}

impl <T:DataGenerator>GenericTokenizer<T> {
    pub fn new(batch:T, tokenizer_name:String) -> Self {
        let tokenizer = tokenizer_wrapper::get_tokenizer(tokenizer_name).unwrap();

        Self {          
            tokenizer: tokenizer,
            batch:batch
        }
        
    }

    pub fn create(&mut self, data:impl TextSupplier<Label=T::Label>) -> Option<T> {
        //let temp = data.labels();   
        let encoding = self.tokenizer.encode(data.text().into());
        let result = self.batch.put_data(&encoding, &data.labels());
        if result {
            return self.get_working_batch();
        }
        None
    }

}


impl <T:DataGenerator> Batcher for GenericTokenizer<T> where {
    
    type S = T;
    type T = T;



    fn create_sync_batch(&mut self, _data:Self::S) -> Option<Self::T> {
            //log::info!("Data {:?}", data);
        

        //let encoding = self.tokenizer.encode_new(data.text());
        //let result = self.batch.put_data(&encoding, &data.labels());
        //if result {
        //    return self.get_working_batch();
        //}
        return None;
            
    }

    fn get_working_batch(&mut self) -> Option<Self::T> {
        let mut old_batch = self.batch.new_data(); 
        std::mem::swap(&mut self.batch, &mut old_batch);

        return Some(old_batch);
    }

}

