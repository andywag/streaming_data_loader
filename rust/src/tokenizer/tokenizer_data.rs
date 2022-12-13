use serde::{Serialize, Deserialize};


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TokenizedData {
    pub ids:Vec<u32>,
    pub positions:Vec<u32>,
    pub attention_mask:Vec<Vec<u32>>,
    pub gaps:Vec<usize>
}