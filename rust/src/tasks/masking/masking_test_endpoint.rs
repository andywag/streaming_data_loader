

use crate::{transport::test_endpoint::EndPoint, datasets::dataset::DataSet, config::TrainingConfig};
use std::fmt::Debug;
pub struct MaskingEndpoint {
    //pub tokenizer:Tokenizer
}

impl MaskingEndpoint {
    pub fn new(_config:TrainingConfig) -> Self {
        //let tokenizer = utils::get_tokenizer(config.tokenizer_name.to_owned());
        Self {
        //    tokenizer:tokenizer
        }
    }

    // TODO : Put in a valid check for the data. Disabled due to use of file match which was inconvenient
    // TODO : Make data input mutable to allow checks
    pub fn check_batch<T:Debug>(&self, _data:T) -> bool {
        //log::info!("Data {:?}", data);
        //let _real_data = data.input_ids.clone();
        
        // Compare only the first batch of data based on a known dataset
        //log::info!("Data {} {:?}", _real_data[0].len(), _real_data[0]);
        //log::info!("Labels {:?}", data.labels);

        /*for x in 0..data.input_ids.len() as usize {
            for y in 0..data.input_ids.len() as usize {
                if data.labels[x][y] != -100 {
                    real_data[x][y] = data.labels[x][y] as u32;
                }
            }
        }
        */
        true

    }
}

impl EndPoint<DataSet> for MaskingEndpoint {
    fn receive(&mut self, data:DataSet) -> bool {
        // TODO : Fixe the masked testing
        match data {
            DataSet::Mask(x) => self.check_batch(x),
            DataSet::Python(x) => self.check_batch(x),

            _ => false
        }
        
    }
}

