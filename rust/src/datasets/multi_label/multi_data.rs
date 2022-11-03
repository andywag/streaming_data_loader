
use abomonation_derive::Abomonation;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, Abomonation)]
pub struct MultiData {
    pub input_ids:Vec<Vec<u32>>,
    pub attention_mask:Vec<Vec<u32>>,
    pub token_type_ids:Vec<Vec<u32>>,
    
    pub labels:Vec<Vec<f32>>
}

impl MultiData {
    pub fn new(batch_size:u32, sequence_length:u32, num_labels:u32) -> Self{
        Self {
            input_ids: vec![vec![0;sequence_length as usize];batch_size as usize],
            attention_mask: vec![vec![0;sequence_length as usize];batch_size as usize],
            token_type_ids: vec![vec![0;sequence_length as usize];batch_size as usize],
            labels: vec![vec![0.0;num_labels as usize];batch_size as usize],
        }
    }
}

#[derive(Debug, Clone)]
pub struct MultiTransport {
    pub text:String,
    pub labels:Vec<u32>,
}

