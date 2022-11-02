use std::cmp::min;

use tokenizers::{Tokenizer};
use crate::batcher::Batcher;

use crate::utils;

use super::SquadConfig;
use super::squad_data::{SquadData, SquadGeneral};

fn find_offset(index:usize, types:&[u32], offsets:&[(usize,usize)]) -> usize {
    let mut sp = 0;
    let mut ep = offsets.len();
    while ep > sp {
        let cp = sp + (ep - sp)/2;
        if offsets[cp] == (0,0) {
            if cp == 0 {
                return 0;
            }
            ep = cp;
        }
        else if types[cp] == 0 {
            sp = cp + 1;
        }
        else if offsets[cp].0 <= index && index <= offsets[cp].1 {
            //println!("Found {} {} {:?}", cp, index, offsets[cp]);
            return cp;
        }
        else if offsets[cp].1 < index {
            sp = cp+1;
        } 
        else {
            ep = cp-1;
        }
    }
    //println!("Found {} {} {:?}", sp, ep, offsets[sp]);
    return sp;
}

pub struct SquadTokenizer {
    pub batch_size:u32,
    pub sequence_length:u32,
    tokenizer:Tokenizer,
    batch:SquadData,
    index:usize
}

impl SquadTokenizer {
    pub fn new(config:&SquadConfig) -> Self {
        let tokenizer = utils::get_tokenizer(config.tokenizer_name.to_owned());
        Self {
            batch_size: config.batch_size,
            sequence_length: config.sequence_length,
            tokenizer: tokenizer,
            batch:SquadData::new(config.batch_size, config.sequence_length),
            index:0
        }
    }

    fn create_data(&self) -> SquadData {
        return SquadData::new(self.batch_size, self.sequence_length)
    }
}

    impl Batcher for SquadTokenizer {
        type S = SquadGeneral;
        type T = SquadData;

        fn create_sync_batch(&mut self, data:SquadGeneral) -> Option<SquadData> {
            let result = self.tokenizer.encode((data.question, data.context), true).unwrap();
    
            let length = min(result.len(), self.sequence_length as usize);
            self.batch.input_ids[self.index][0..length].clone_from_slice(&result.get_ids()[0..length]);
            self.batch.token_type_ids[self.index][0..length].clone_from_slice(&result.get_type_ids()[0..length]);
            self.batch.attention_mask[self.index][0..length].clone_from_slice(&result.get_attention_mask()[0..length]);
            self.batch.answers[self.index] = data.answer.clone();
            
            //println!("Offsets {:?}", result.get_offsets());
    
            let mut start = find_offset(data.sp as usize, result.get_type_ids(), result.get_offsets());
            let mut end = find_offset(data.ep as usize, result.get_type_ids(), result.get_offsets());
    
            let ans_token = self.tokenizer.encode(data.answer.unwrap(), false);
            let ans_ids = ans_token.unwrap();
    
            //println!("HHHH {} {} {:?}", start, self.sequence_length, self.batch.answers[self.index]);
            if start > self.sequence_length as usize{
                return None;
            }
            if self.batch.input_ids[self.index].len() > start && ans_ids.get_ids()[0] != self.batch.input_ids[self.index][start as usize] {
                // TODO : Hacked way of searching for the proper start/end points
                // Doesn't catch all cases which run through the continue loop
                // Misses some mismatch cases because only works on first char

                for _ in 0..10 {
                    start += 1;
                    end += 1;
                    if self.batch.input_ids[self.index].len() > start {
                        if ans_ids.get_ids()[0] == self.batch.input_ids[self.index][start as usize] {
                            break;
                        }
                    }
                }
                if ans_ids.get_ids()[0] != self.batch.input_ids[self.index][start as usize] {
                    return None;
                }
                if start >= self.sequence_length as usize || end >= self.sequence_length as usize {
                    return None;
                }
                
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



