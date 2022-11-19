
use serde::{Serialize, Deserialize};
use std::cmp::min;

use super::MultiConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiData {
    pub input_ids:Vec<Vec<u32>>,
    pub attention_mask:Vec<Vec<u32>>,
    pub token_type_ids:Vec<Vec<u32>>,
    pub labels:Vec<Vec<f32>>,
    config:MultiConfig,

    pub index:usize
}

impl MultiData {
    pub fn new(config:&MultiConfig) -> Self{
        Self {
            input_ids: vec![vec![0;config.sequence_length as usize];config.batch_size as usize],
            attention_mask: vec![vec![0;config.sequence_length as usize];config.batch_size as usize],
            token_type_ids: vec![vec![0;config.sequence_length as usize];config.batch_size as usize],
            labels: vec![vec![0.0;config.number_labels as usize];config.batch_size as usize],
            config:config.clone(),

            index:0,
        }
    }

    pub fn new_data(&self) -> Self {
        Self::new(&self.config)
    }


    pub fn put_data(&mut self, result:&tokenizers::Encoding, labels:Vec<u32>) -> bool {

        let ids = result.get_ids();
        let length = min(ids.len(), self.config.sequence_length as usize);
        self.input_ids[self.index][0..length].clone_from_slice(&ids[0..length]);
        self.token_type_ids[self.index][0..length].clone_from_slice(&result.get_type_ids()[0..length]);
        self.attention_mask[self.index][0..length].clone_from_slice(&result.get_attention_mask()[0..length]);
        for x in labels {
            self.labels[self.index][x as usize] = 1.0;
        }
        self.index += 1;
        self.done()
    }

    pub fn done(&self) -> bool {
        self.index == self.config.batch_size
    }
}

#[derive(Debug, Clone)]
pub struct MultiTransport {
    pub text:String,
    pub labels:Vec<u32>,
}

