
use serde::{Serialize, Deserialize};
use rand::prelude::SliceRandom;
use rand::thread_rng;

use super::MaskingConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaskedData {
    pub input_ids:Vec<Vec<u32>>,
    pub attention_mask:Vec<Vec<u32>>,
    pub labels:Vec<Vec<i32>>,
    index:usize,

    config:MaskingConfig,
    batch_size:usize,
    sequence_length:usize,
    masked_length:usize,
    attention_base:Vec<u32>,
    position_base:Vec<u32>,
    mask:u32
}

impl MaskedData {
    pub fn new(config:MaskingConfig, mask:u32) -> Self{
        let position_base:Vec<u32> = (0..config.sequence_length as u32).collect();
        let batch_size = config.batch_size;
        let sequence_length = config.sequence_length;
        let mask_length = config.mask_length;

        Self {
            input_ids: vec![vec![0;config.sequence_length];config.batch_size],
            attention_mask: vec![vec![1;config.sequence_length];config.batch_size],
            labels:vec![vec![-100;config.sequence_length]; config.batch_size],
            index:0, 

            config:config,
            batch_size:batch_size,
            sequence_length:sequence_length,
            masked_length:mask_length,
            attention_base:vec![0;sequence_length as usize],
            position_base:position_base,
            mask:mask
        }
    }

    pub fn new_data(&mut self) -> Self {
        MaskedData::new(self.config.clone(), self.mask)
    }

    pub fn mask_batch(&mut self) {

        self.position_base.shuffle(&mut thread_rng());

        for x in 0..self.masked_length as usize {
            if self.input_ids[self.index][self.position_base[x] as usize] != 0 {
                self.labels[self.index][self.position_base[x] as usize] = self.input_ids[self.index][self.position_base[x] as usize] as i32;
                self.input_ids[self.index][self.position_base[x] as usize] = self.mask;       
            }
        }
        
    }

    pub fn put_data(&mut self, ids:&[u32]) -> bool{
        self.input_ids[self.index][0..ids.len() as usize].clone_from_slice(ids);
        if ids.len() < self.sequence_length {
            let s = self.sequence_length;
            self.attention_mask[self.index][(s-ids.len())..s].copy_from_slice(&self.attention_base[(s-ids.len())..s]);
        }

        self.mask_batch();
        self.index += 1;
        self.done()
    }

    pub fn done(&self) -> bool{
        self.index == self.input_ids.len()
       
    }


}