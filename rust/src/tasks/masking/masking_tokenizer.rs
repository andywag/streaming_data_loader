use tokenizers::{Tokenizer};
use rand::seq::SliceRandom;
use rand::thread_rng;
use crate::batcher::Batcher;
use crate::utils;
use super::masked_data::MaskedData;
use super::MaskingConfig;

pub struct BaseTokenizer {
    pub batch_size:u32,
    pub sequence_length:u32,
    pub mask_length:u32,
    tokenizer:Tokenizer,
    store_original:bool,
    batch:MaskedData,
    index:usize,
    attention_mask:Vec<u32>,
    last_ids:Option<Vec<u32>>,
    mask:u32,
    cls:u32,
    sep:u32,
    pad:u32
}
 
impl BaseTokenizer {
    pub fn new(config:&MaskingConfig, store_original:bool) -> Self {
        let mask_length = config.mask_length;
        let tokenizer = utils::get_tokenizer(config.tokenizer_name.to_owned());
        
        
        let location =  config.tokenizer_name.find("roberta");
        let tokens = match location {
            Some(_) => {
                (tokenizer.token_to_id("<s>").unwrap().to_owned(),
                tokenizer.token_to_id("</s>").unwrap().to_owned(),
                tokenizer.token_to_id("<mask>").unwrap().to_owned(),
                tokenizer.token_to_id("<pad>").unwrap().to_owned())
            },
            None => {
                (tokenizer.token_to_id("[CLS]").unwrap().to_owned(),
                 tokenizer.token_to_id("[SEP]").unwrap().to_owned(),
                 tokenizer.token_to_id("[MASK]").unwrap().to_owned(),
                 tokenizer.token_to_id("[PAD]").unwrap().to_owned())
               
            }
        };

        

        Self {
            batch_size: config.batch_size,
            sequence_length: config.sequence_length,
            mask_length: mask_length,
            tokenizer: tokenizer,
            store_original: store_original,
            batch:MaskedData::new(config.batch_size, config.sequence_length, mask_length, tokens.3),
            index:0, 
            attention_mask:vec![0;config.sequence_length as usize],
            last_ids:None, 
            cls:tokens.0,
            sep:tokens.1,
            mask:tokens.2,
            pad:tokens.3
        }
    }

    fn create_data(&self) -> MaskedData {
        return MaskedData::new(self.batch_size, self.sequence_length, self.mask_length, self.pad)
    }
     
   
    pub fn mask_batch(&self, batch:&mut MaskedData, mask:u32) {
        let mut positions:Vec<u32> = (0..self.sequence_length).collect();

        for index in 0..self.batch_size as usize {
            positions.shuffle(&mut thread_rng());

            for x in 0..self.mask_length as usize {
                if batch.input_ids[index][positions[x] as usize] != 0 {
                    batch.labels[index][positions[x] as usize] = batch.input_ids[index][positions[x] as usize] as i32;
                    batch.input_ids[index][positions[x] as usize] = mask;       
                }
            }
        }
    }

}

// TODO : Handle Batch overlap : Currently throws out remaining sequence at the end of the batch
impl Batcher for BaseTokenizer {
    type S = String;
    type T = MaskedData;

    fn create_sync_batch(&mut self, data:Self::S) -> Option<Self::T> {
        let result = self.tokenizer.encode(data, true).unwrap();
            //let ids = result.get_ids();
            let s = self.sequence_length as usize;
            
            let mut new_ids:Option<Vec<u32>> = None;
            std::mem::swap(&mut self.last_ids, &mut new_ids);

            let ids = match new_ids {
                Some(mut x) => {
                    x.push(self.sep);
                    x.push(self.sep);
                    [x, result.get_ids().to_vec()].concat().to_vec()
                },
                None => result.get_ids().to_vec(),
            };


            let mut current_index = 0;
            while current_index < ids.len() {
                //let mut length:usize;// = self.sequence_length as usize;
                self.batch.input_ids[self.index][0] = self.cls; // Always put a CLS in token location 0
                if (current_index + s - 1) < ids.len() {
                    self.batch.input_ids[self.index][1..s as usize].clone_from_slice(&ids[current_index..current_index+s-1]);
                }
                else {
                    let  length = ids.len() - current_index;
                    self.batch.input_ids[self.index][1..length].clone_from_slice(&ids[current_index..current_index+length-1]);
                    if length < s {
                        self.batch.input_ids[self.index][length] = self.sep;
                    }
                    self.batch.attention_mask[self.index][(s-length+1)..s].copy_from_slice(&self.attention_mask[(s-length+1)..s]);
                }

                current_index += s - 1;
                self.index += 1;

                if self.index == self.batch_size as usize {
                    
                    if current_index < ids.len() {
                        self.last_ids = Some(ids.clone()[current_index..ids.len()].to_vec());
                    }
                    
                    let mut old_batch = self.create_data(); 
                    std::mem::swap(&mut self.batch, &mut old_batch);
                    self.index = 0;
                    if self.store_original {
                        old_batch.original = Some(old_batch.input_ids.clone());
                    }
                    self.mask_batch(&mut old_batch, self.mask);
                    return Some(old_batch);
                }
            }
            self.last_ids = None;
            return None;
    }

    fn get_working_batch(&mut self) -> Option<Self::T> {
        let mut old_batch = self.create_data(); 
        std::mem::swap(&mut self.batch, &mut old_batch);
        self.index = 0;
        if self.store_original {
            old_batch.original = Some(old_batch.input_ids.clone());
        }
        self.mask_batch(&mut old_batch,  self.mask);
        return Some(old_batch);
    }

}


