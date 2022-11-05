
use tokenizers::Tokenizer;

use crate::{tasks::masking::{MaskingConfig, masked_data::MaskedData}, utils, test_endpoint::EndPoint};

pub struct MaskingEndpoint {
    pub tokenizer:Tokenizer
}

impl MaskingEndpoint {
    pub fn new(config:MaskingConfig) -> Self {
        let tokenizer = utils::get_tokenizer(config.tokenizer_name.to_owned());
        Self {
            tokenizer:tokenizer
        }
    }

    // TODO : Put in a valid check for the data. Disabled due to use of file match which was inconvenient
    // TODO : Make data input mutable to allow checks
    pub fn check_batch(&self, data:MaskedData) -> bool {
        let mut real_data = data.input_ids.clone();
        //log::info!("Here I am to save teh day {:?}", data);
        // Compare only the first batch of data based on a known dataset
        for x in 0..data.input_ids.len() as usize {
            for y in 0..data.input_ids.len() as usize {
                if data.labels[x][y] != -100 {
                    real_data[x][y] = data.labels[x][y] as u32;
                }
            }
        }
        match data.original {
            Some(original) => {
                for x in 0..data.input_ids.len() as usize {
                    for y in 0..data.input_ids.len() as usize {
                        if real_data[x][y] != original[x][y] {
                            log::error!("First Mismatch {x} {y} {} {}", real_data[x][y], original[x][y]);
                            log::info!("First {:?}", data.input_ids[x]);
                            log::info!("Real {:?}", real_data[x]);
                            log::info!("Original {:?}", original[x]);
                        
                            return false;
                        }
                    }
                }
                return true;
            },
            None => {
                log::error!("Original Data Not Sent for Comparison");
                return false;
            },
        }

    }
}

impl EndPoint<MaskedData> for MaskingEndpoint {
    fn receive(&mut self, data:MaskedData) -> bool {
        return self.check_batch(data);
    }
}

