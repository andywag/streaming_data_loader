use tokenizers::{Tokenizer};
use rand::seq::SliceRandom;
use rand::thread_rng;
use crate::batcher::Batcher;
use crate::utils;
use super::masked_data::MaskedData;
use super::masking_config::MaskingConfig;

pub struct BaseTokenizer {
    pub batch_size:u32,
    pub sequence_length:u32,
    pub mask_length:u32,
    tokenizer:Tokenizer,
    batch:MaskedData,
    index:usize,
    attention_mask:Vec<u32>,
    positions:Vec<u32>, 
    mask:u32,
    _sep:u32,
    _cls:u32
}
 
impl BaseTokenizer {
    pub fn new(config:&MaskingConfig) -> Self {
        let mask_length = config.mask_length;
        let tokenizer = utils::get_tokenizer(config.tokenizer_name.to_owned());
        let mask = tokenizer.token_to_id("[MASK]").unwrap().to_owned();
        let cls = tokenizer.token_to_id("[CLS]").unwrap().to_owned();
        let sep = tokenizer.token_to_id("[SEP]").unwrap().to_owned();

        Self {
            batch_size: config.batch_size,
            sequence_length: config.sequence_length,
            mask_length: mask_length,
            tokenizer: tokenizer,
            batch:MaskedData::new(config.batch_size, config.sequence_length, mask_length),
            index:0, 
            attention_mask:vec![0;config.sequence_length as usize],
            positions:(0..config.sequence_length).collect(),
            mask:mask,
            _cls:cls,
            _sep:sep,
        }
    }

    fn create_data(&self) -> MaskedData {
        return MaskedData::new(self.batch_size, self.sequence_length, self.mask_length)
    }
     
    pub fn mask_sequence(&self, index:usize, positions:&Vec<u32>, batch:&mut MaskedData, l:usize, mask:u32) {
        for x in 0..self.mask_length as usize {
            if positions[x] <= l as u32 {
                batch.masked_lm_positions[index][x] = positions[x];
                let masked_value = batch.input_ids[index][positions[x] as usize];
                batch.input_ids[index][positions[x] as usize] = mask;
                batch.masked_lm_positions[index][x] = positions[x];
                batch.masked_lm_labels[index][x] = masked_value;
            }
            
        }
    }

    // TODO : l input is incorrect 
    pub fn mask_batch(&self, batch:&mut MaskedData, l:usize, mask:u32) {
        let mut positions:Vec<u32> = (0..self.sequence_length).collect();

        for index in 0..self.batch_size as usize {
            positions.shuffle(&mut thread_rng());

            for x in 0..self.mask_length as usize {
                if positions[x] <= l as u32 {
                    batch.masked_lm_positions[index][x] = positions[x];
                    let masked_value = batch.input_ids[index][positions[x] as usize];
                    batch.input_ids[index][positions[x] as usize] = mask;
                    batch.masked_lm_positions[index][x] = positions[x];
                    batch.masked_lm_labels[index][x] = masked_value;
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
            let ids = result.get_ids();
            
            let mut current_index = 0;
            while current_index < ids.len() {
                self.positions.shuffle(&mut thread_rng());
                let mut length:usize = self.sequence_length as usize;
                if (current_index + self.sequence_length as usize) < ids.len() {
                    
                    self.batch.input_ids[self.index].clone_from_slice(&ids[current_index..current_index+self.sequence_length as usize]);
                }
                else {
                    length = ids.len() - current_index;
                    self.batch.input_ids[self.index][0..length].clone_from_slice(&ids[current_index..current_index+length as usize]);
                    self.batch.attention_mask[self.index][(self.sequence_length as usize-length)..self.sequence_length as usize].copy_from_slice(&self.attention_mask[0..length]);
                }


                current_index += self.sequence_length as usize;
                self.index += 1;

                if self.index == self.batch_size as usize {
                    let mut old_batch = self.create_data(); 
                    std::mem::swap(&mut self.batch, &mut old_batch);
                    self.index = 0;
                    self.mask_batch(&mut old_batch, length, self.mask);
                    return Some(old_batch);
                }
            }
            return None;
    }

    fn get_working_batch(&mut self) -> Option<Self::T> {
        let mut old_batch = self.create_data(); 
        std::mem::swap(&mut self.batch, &mut old_batch);
        self.index = 0;
        self.mask_batch(&mut old_batch, self.sequence_length as usize, self.mask);
        return Some(old_batch);
    }

}


