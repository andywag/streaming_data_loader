
use abomonation_derive::Abomonation;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, Abomonation)]
pub struct MaskedData {
    pub input_ids:Vec<Vec<u32>>,
    pub attention_mask:Vec<Vec<u32>>,
    pub masked_lm_positions:Vec<Vec<u32>>,
    pub masked_lm_labels:Vec<Vec<u32>>
}

impl MaskedData {
    pub fn new(batch_size:u32, sequence_length:u32, mask_length:u32) -> Self{
        Self {
            input_ids: vec![vec![0;sequence_length as usize];batch_size as usize],
            attention_mask: vec![vec![1;sequence_length as usize];batch_size as usize],
            masked_lm_positions: vec![vec![0;mask_length as usize];batch_size as usize],
            masked_lm_labels: vec![vec![0;mask_length as usize];batch_size as usize],
        }
    }
}