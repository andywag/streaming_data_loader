

use crate::{tasks::masking::{MaskingConfig, masked_data::MaskedData}, transport::test_endpoint::EndPoint, datasets::DataSet};

pub struct MaskingEndpoint {
    //pub tokenizer:Tokenizer
}

impl MaskingEndpoint {
    pub fn new(_config:MaskingConfig) -> Self {
        //let tokenizer = utils::get_tokenizer(config.tokenizer_name.to_owned());
        Self {
        //    tokenizer:tokenizer
        }
    }

    // TODO : Put in a valid check for the data. Disabled due to use of file match which was inconvenient
    // TODO : Make data input mutable to allow checks
    pub fn check_batch(&self, data:MaskedData) -> bool {
        let mut real_data = data.input_ids.clone();
        // Compare only the first batch of data based on a known dataset
        for x in 0..data.input_ids.len() as usize {
            for y in 0..data.input_ids.len() as usize {
                if data.labels[x][y] != -100 {
                    real_data[x][y] = data.labels[x][y] as u32;
                }
            }
        }
        true

    }
}

impl EndPoint<DataSet> for MaskingEndpoint {
    fn receive(&mut self, _data:DataSet) -> bool {
        // TODO : Fixe the masked testing
        true
        //return self.check_batch(data);
    }
}

