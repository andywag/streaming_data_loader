
use abomonation_derive::Abomonation;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, Abomonation)]
pub struct SquadData {
    pub input_ids:Vec<Vec<u32>>,
    pub attention_mask:Vec<Vec<u32>>,
    pub token_type_ids:Vec<Vec<u32>>,
    pub start_positions:Vec<u32>,
    pub end_positions:Vec<u32>,
    pub answers:Vec<Option<String>>
}

impl SquadData {
    pub fn new(batch_size:u32, sequence_length:u32) -> Self{
        Self {
            input_ids: vec![vec![0;sequence_length as usize];batch_size as usize],
            attention_mask: vec![vec![1;sequence_length as usize];batch_size as usize],
            token_type_ids: vec![vec![1;sequence_length as usize];batch_size as usize],
            start_positions: vec![0;batch_size as usize],
            end_positions: vec![0;batch_size as usize],
            answers:vec![None;batch_size as usize]
        }
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

