use std::cmp::min;

use serde::{Serialize, Deserialize};

use super::SingleClassConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SingleClassData {
    pub input_ids:Vec<Vec<u32>>,
    pub attention_mask:Vec<Vec<u32>>,
    pub token_type_ids:Vec<Vec<u32>>,
    pub label:Vec<u32>,
    config:SingleClassConfig,

    index:usize
}

impl SingleClassData {
    pub fn new(config:&SingleClassConfig) -> Self{
        Self {
            input_ids: vec![vec![0;config.sequence_length ];config.batch_size],
            attention_mask: vec![vec![0;config.sequence_length];config.batch_size ],
            token_type_ids: vec![vec![0;config.sequence_length ];config.batch_size],
            label: vec![0;config.batch_size as usize],
            config:config.clone(),

            index:0
        }
    }

    pub fn new_data(&mut self) -> Self {
        Self::new(&self.config)
    }

    pub fn put_data(&mut self, result:&tokenizers::Encoding, label:u32) -> bool {

        let ids = result.get_ids();
        let length = min(ids.len(), self.config.sequence_length);
        self.input_ids[self.index][0..length].clone_from_slice(&ids[0..length]);
        self.token_type_ids[self.index][0..length].clone_from_slice(&result.get_type_ids()[0..length]);
        self.attention_mask[self.index][0..length].clone_from_slice(&result.get_attention_mask()[0..length]);
        self.label[self.index] = label;

        self.index += 1;
        self.done()
    }

    pub fn done(&self) -> bool {
        self.index == self.config.batch_size
    }

}

#[derive(Debug, Clone)]
pub struct SingleClassTransport {
    pub text:String,
    pub label:u32,
}

