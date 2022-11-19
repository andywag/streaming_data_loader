
use serde::{Serialize, Deserialize};
use std::cmp::min;

use super::SquadConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SquadData {
    pub input_ids:Vec<Vec<u32>>,
    pub attention_mask:Vec<Vec<u32>>,
    pub token_type_ids:Vec<Vec<u32>>,
    pub start_positions:Vec<u32>,
    pub end_positions:Vec<u32>,
    pub answers:Vec<Option<String>>,

    config:SquadConfig,

    index:usize,
}


impl SquadData {
    pub fn new(config:&SquadConfig) -> Self{
        let sequence_length = config.sequence_length as usize;
        let batch_size = config.batch_size as usize;

        Self {
            
            input_ids: vec![vec![0;sequence_length as usize];batch_size as usize],
            attention_mask: vec![vec![1;sequence_length as usize];batch_size as usize],
            token_type_ids: vec![vec![1;sequence_length as usize];batch_size as usize],
            start_positions: vec![0;batch_size as usize],
            end_positions: vec![0;batch_size as usize],
            answers:vec![None;batch_size as usize],
            config:config.clone(),

            index:0
            
        }
    }

    pub fn new_data(&self) -> Self {
        SquadData::new(&self.config)
    }

    pub fn put_data(&mut self, result:&tokenizers::Encoding, data:SquadGeneral) -> bool {

        let length = min(result.len(), self.config.sequence_length as usize);
        self.input_ids[self.index][0..length].clone_from_slice(&result.get_ids()[0..length]);
        self.token_type_ids[self.index][0..length].clone_from_slice(&result.get_type_ids()[0..length]);
        self.attention_mask[self.index][0..length].clone_from_slice(&result.get_attention_mask()[0..length]);
        self.answers[self.index] = data.answer.clone();
            
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

                
        // Condition to catch potential errors and continue operation
        match (start_token, end_token) {
            (Some(_), Some(_)) => {},
            _ => {
                log::info!("Error with Data");
                return false
            }
        }

        self.start_positions[self.index] = start_token.unwrap() as u32;
        self.end_positions[self.index] = end_token.unwrap() as u32;


        self.index += 1;
        self.done()
    }

    pub fn done(&self) -> bool {
        self.index == self.config.batch_size
    }
    
}


#[derive(Debug, Clone)]
pub struct SquadGeneral {
    pub question:String,
    pub context:String,
    pub sp:u32,
    pub ep:u32,
    pub offset:Option<usize>,
    pub answer:Option<String>
}

