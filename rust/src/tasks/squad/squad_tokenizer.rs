use std::cmp::min;

use tokenizers::{Tokenizer};
use crate::batcher::Batcher;

use crate::utils;

use super::SquadConfig;
use super::squad_data::{SquadData, SquadGeneral};

/* 
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
*/

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
            
            let mut start_token:Option<usize> = None;
            let mut end_token:Option<usize> = None;
            // Kludgey code to search through offsets to find the proper offsets
            // Issue due to rusts handling of characters
            match data.offset {
                Some(offset) => {
                    let offsets = result.get_offsets();
                    for x in 0..offset+1 {
                        start_token = result.char_to_token((data.sp) as usize +x, 1);
                        end_token = result.char_to_token((data.ep-1) as usize + x, 1);
                        match (start_token, end_token) {
                            (Some(s), Some(e)) => {
                                //log::info!("BBBB {} {} : {} {} {:?} {:?}", data.sp, data.ep, s, e, offsets[s], offsets[e]);
                                if offsets[s].0 == data.sp as usize + x && offsets[e].1 == data.ep as usize + x{ 
                                    break;
                                }
                            },
                            _ => {}
                        }
                    }
                },
                None => {
                    start_token = result.char_to_token(data.sp as usize, 1);
                    end_token = result.char_to_token((data.ep-1) as usize, 1);
                },
            }

            //let d_vec = self.batch.input_ids[self.index][start_token.unwrap() as usize..end_token.unwrap()+1 as usize].to_owned();
            //let result_str = self.tokenizer.decode(d_vec, true).unwrap(); 
            //log::info!("Running {:?} {:?}", data.answer, result_str);
            
            // Condition to catch potential errors and continue operation
            match (start_token, end_token) {
                (Some(_), Some(_)) => {},
                _ => {
                    log::info!("Error with Data");
                    return None
                }
            }

            self.batch.start_positions[self.index] = start_token.unwrap() as u32;
            self.batch.end_positions[self.index] = end_token.unwrap() as u32;

            self.index += 1;
            if self.index == self.batch_size as usize {
                return self.get_working_batch();
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



