
use serde::ser::SerializeStruct;
use serde::{Serialize, Deserialize};
use rand::prelude::SliceRandom;
use rand::thread_rng;

use crate::batcher::BatchConfig;

use super::MaskingConfig;

#[derive(Debug, Clone, Deserialize)]
pub struct MaskedData {
    pub input_ids:Vec<Vec<u32>>,
    pub attention_mask:Vec<Vec<u32>>,
    pub labels:Vec<Vec<i32>>,
    index:usize,

    config:MaskingConfig,
    batch_config:BatchConfig,

    masked_length:usize,
    attention_base:Vec<u32>,
    position_base:Vec<u32>,
    mask:u32
}

impl MaskedData {
    pub fn new(config:MaskingConfig, batch_config:BatchConfig, mask:u32) -> Self{
        let position_base:Vec<u32> = (0..batch_config.sequence_length as u32).collect();
        let sequence_length = batch_config.sequence_length;
        let mask_length = config.mask_length;

        Self {
            input_ids: vec![vec![0;batch_config.sequence_length];batch_config.batch_size],
            attention_mask: vec![vec![1;batch_config.sequence_length];batch_config.batch_size],
            labels:vec![vec![-100;batch_config.sequence_length]; batch_config.batch_size],
            index:0, 

            config:config,
            batch_config:batch_config,
            masked_length:mask_length,
            attention_base:vec![0;sequence_length as usize],
            position_base:position_base,
            mask:mask
        }
    }

    pub fn new_data(&mut self) -> Self {
        MaskedData::new(self.config.clone(), self.batch_config.clone(), self.mask)
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
        if ids.len() < self.batch_config.sequence_length {
            let s = self.batch_config.sequence_length;
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

impl Serialize for MaskedData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
            let mut state = serializer.serialize_struct("MaskedData", 3)?;
            state.serialize_field("input_ids", &self.input_ids)?;
            state.serialize_field("attention_mask", &self.attention_mask)?;
            state.serialize_field("labels", &self.labels)?;
            state.end()
    }
}