use std::cmp::min;

use tokenizers::{Tokenizer};
use crate::batcher::Batcher;

use crate::utils;

use super::MultiConfig;
use super::multi_data::{MultiData, MultiTransport};


pub struct MultiTokenizer {
    pub batch_size:u32,
    pub sequence_length:u32,
    pub number_labels:u32,
    tokenizer:Tokenizer,
    batch:MultiData,
    index:usize
}

impl MultiTokenizer {
    pub fn new(config:&MultiConfig) -> Self {
        let tokenizer = utils::get_tokenizer(config.tokenizer_name.to_owned());
        Self {
            batch_size: config.batch_size,
            sequence_length: config.sequence_length,
            number_labels: config.number_labels,
            tokenizer: tokenizer,
            batch:MultiData::new(config.batch_size, config.sequence_length, config.number_labels),
            index:0
        }
    }

    fn create_data(&self) -> MultiData {
        return MultiData::new(self.batch_size, self.sequence_length, self.number_labels);
    }
}

    impl Batcher for MultiTokenizer {
        type S = MultiTransport;
        type T = MultiData;

        fn create_sync_batch(&mut self, data:Self::S) -> Option<Self::T> {
            //println!("Data {:?}", data);
            let result = self.tokenizer.encode(data.text, true).unwrap();
    
            let length = min(result.len(), self.sequence_length as usize);
            self.batch.input_ids[self.index][0..length].clone_from_slice(&result.get_ids()[0..length]);
            self.batch.token_type_ids[self.index][0..length].clone_from_slice(&result.get_type_ids()[0..length]);
            self.batch.attention_mask[self.index][0..length].clone_from_slice(&result.get_attention_mask()[0..length]);
            for x in data.labels {
                self.batch.labels[self.index][x as usize] = 1.0;
            }
                        
            //println!("Here {} {}", self.index, self.batch_size);
            self.index += 1;
            if self.index == self.batch_size as usize {
                let mut old_batch = self.create_data(); 
                std::mem::swap(&mut self.batch, &mut old_batch);
                self.index = 0;
                return Some(old_batch);
            }
            return None;
    }

    fn get_working_batch(&mut self) -> Option<Self::T> {
        if self.index == 0 {
            return None;
        }
        let mut old_batch = self.create_data(); 
        std::mem::swap(&mut self.batch, &mut old_batch);
        self.index = 0;
        return Some(old_batch);
    }

}

