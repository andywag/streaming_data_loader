
use abomonation_derive::Abomonation;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, Abomonation)]
pub struct SingleClassData {
    pub input_ids:Vec<Vec<u32>>,
    pub attention_mask:Vec<Vec<u32>>,
    pub token_type_ids:Vec<Vec<u32>>,
    
    pub labels:Vec<f32>
}

impl SingleClassData {
    pub fn new(batch_size:u32, sequence_length:u32) -> Self{
        Self {
            input_ids: vec![vec![0;sequence_length as usize];batch_size as usize],
            attention_mask: vec![vec![0;sequence_length as usize];batch_size as usize],
            token_type_ids: vec![vec![0;sequence_length as usize];batch_size as usize],
            labels: vec![0.0;batch_size as usize],
        }
    }
}

#[derive(Debug, Clone)]
pub struct SingleClassTransport {
    pub text:String,
    pub label:u32,
}

