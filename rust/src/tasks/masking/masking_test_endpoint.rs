

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
        log::info!("Data {:?}", _data);
        true

    }
}

impl EndPoint<DataSet> for MaskingEndpoint {
    fn receive(&mut self, data:DataSet) -> bool {
        // TODO : Fixe the masked testing
        match data {
            DataSet::BertHier(x) => self.check_batch(x),
            DataSet::Bert(x) => self.check_batch(x),
            
            x => self.check_batch(x)
        }
        
    }
}

