pub mod masked_data;
pub mod masking_tokenizer;
pub mod masking_test_endpoint;
pub mod masking_runner;
pub mod gpt2_tokenizer;

use serde::{Serialize, Deserialize};


#[derive(Clone, Serialize, Deserialize)]
pub struct MaskingConfig{
    pub batch_size:u32,
    pub sequence_length:u32,
    pub mask_length:u32,
    pub tokenizer_name:String
}