
use tokenizers::{Tokenizer};
use crate::batcher::Batcher;
use crate::utils;
use super::masked_data::MaskedData;
use super::MaskingConfig;

pub struct GPTTokenizer {
    pub batch_size:u32,
    pub sequence_length:u32,
    pub mask_length:u32,
    tokenizer:Tokenizer,
    store_original:bool,
    batch:MaskedData,
    index:usize,
    attention_mask:Vec<u32>,
    label_mask:Vec<i32>,
    last_ids:Option<Vec<u32>>,
    
}
 
impl GPTTokenizer {
    pub fn new(config:&MaskingConfig, store_original:bool) -> Self {
        let mask_length = config.mask_length;
        let tokenizer = utils::get_tokenizer(config.tokenizer_name.to_owned());
                
        

        Self {
            batch_size: config.batch_size,
            sequence_length: config.sequence_length,
            mask_length: mask_length,
            tokenizer: tokenizer,
            store_original: store_original,
            batch:MaskedData::new(config.batch_size, config.sequence_length, mask_length, 0),
            index:0, 
            attention_mask:vec![0;config.sequence_length as usize],
            label_mask:vec![-100;config.sequence_length as usize],

            last_ids:None, 
            
        }
    }

    fn create_data(&self) -> MaskedData {
        return MaskedData::new(self.batch_size, self.sequence_length, self.mask_length, 0)
    }
     
    fn handle_input_ids(&mut self, ids:Vec<u32>) -> Option<MaskedData> {
        let mut current_index = 0;
        let s = self.sequence_length as usize;

        while current_index < ids.len() {
            if (current_index + s) < ids.len() { // Full Sequence Fits
                self.batch.input_ids[self.index][0..s as usize].clone_from_slice(&ids[current_index..current_index+s]);
                let labels:Vec<i32> = self.batch.input_ids[self.index].clone().into_iter().map(|e| e as i32).collect();
                self.batch.labels[self.index][0..s as usize].clone_from_slice(&labels);

                
            }
            else { // Finish off the sequence
                let  length = ids.len() - current_index;
                self.batch.input_ids[self.index][0..length].clone_from_slice(&ids[current_index..current_index+length]);
                let labels:Vec<i32> = self.batch.input_ids[self.index].clone().into_iter().map(|e| e as i32).collect();

                self.batch.labels[self.index].clone_from_slice(&labels);
                self.batch.labels[self.index][(s-length)..s].copy_from_slice(&self.label_mask[(s-length)..s]);

                self.batch.attention_mask[self.index][(s-length)..s].copy_from_slice(&self.attention_mask[(s-length)..s]);
            }
            current_index += s;
            self.index += 1;
            if self.index == self.batch_size as usize { // Finish Off the Batch
                if current_index < ids.len() {
                    self.last_ids = Some(ids.clone()[current_index..ids.len()].to_vec());
                }
                
                let mut old_batch = self.create_data(); 
                std::mem::swap(&mut self.batch, &mut old_batch);
                self.index = 0;
                if self.store_original {
                    old_batch.original = Some(old_batch.input_ids.clone());
                }
                return Some(old_batch);
            }
        }
        self.last_ids = None;
        return None;
    }

}

// TODO : Handle Batch overlap : Currently throws out remaining sequence at the end of the batch
impl Batcher for GPTTokenizer {
    type S = String;
    type T = MaskedData;

    fn create_sync_batch(&mut self, data:Self::S) -> Option<Self::T> {
        //log::info!("Data {}", data);
        let result = self.tokenizer.encode(data, true).unwrap();
        
            //let ids = result.get_ids();
            
        // The batch size should be large enough to consume this data or the next encoding will be lost
        // ** Batch Size should be chosen to handle this 
        if self.last_ids.is_some() { // Process the last set of data
            let mut new_ids:Option<Vec<u32>> = None;
            std::mem::swap(&mut self.last_ids, &mut new_ids);
            let result = self.handle_input_ids(new_ids.unwrap());
            if result.is_some() {
                return result;
            }
        }
        return self.handle_input_ids(result.get_ids().to_vec());
        
    }

    fn get_working_batch(&mut self) -> Option<Self::T> {
        let mut old_batch = self.create_data(); 
        std::mem::swap(&mut self.batch, &mut old_batch);
        self.index = 0;
        if self.store_original {
            old_batch.original = Some(old_batch.input_ids.clone());
        }
        
        return Some(old_batch);
    }

}


