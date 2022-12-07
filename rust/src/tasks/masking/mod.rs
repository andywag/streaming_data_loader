pub mod masked_data;
//pub mod masking_tokenizer;
pub mod masking_test_endpoint;
pub mod masking_runner;
//pub mod gpt2_tokenizer;
pub mod gpt2_test_endpoint;
pub mod gpt_data;
pub mod t5_data;
pub mod t5_test_endpoint;

pub mod masking_cases;

use serde::{Serialize, Deserialize};


#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct MaskingConfig{
    pub mask_length:usize
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct T5Config{
    pub number_spans:usize,
    pub mask_probability:f64
}