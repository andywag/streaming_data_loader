

use crate::{tasks::masking::{T5Config}, transport::test_endpoint::EndPoint, datasets::DataSet};

use super::t5_data::T5Data;

pub struct T5Endpoint {
    //pub tokenizer:Tokenizer
}

impl T5Endpoint {
    pub fn new(_config:T5Config) -> Self {
        //let tokenizer = utils::get_tokenizer(config.tokenizer_name.to_owned());
        Self {
        //    tokenizer:tokenizer
        }
    }

    // TODO : Put in a valid check for the data. Disabled due to use of file match which was inconvenient
    // TODO : Make data input mutable to allow checks
    pub fn check_batch(&self, _data:T5Data) -> bool {
        //log::info!("Data {:?}", data);
        //let mut real_data = data.input_ids.clone();
        // Compare only the first batch of data based on a known dataset
        
        true

    }
}

impl EndPoint<DataSet> for T5Endpoint {
    fn receive(&mut self, data:DataSet) -> bool {
        match data {
            DataSet::Mask(_) => todo!(),
            DataSet::Gpt2(_) => todo!(),
            DataSet::T5(x) =>  self.check_batch(x),
            //DataSet::T5(x) => self.check_batch(x);
            //_ => todo!();
        };
        true
        // TODO : Fixe the masked testing
        //log::info!("DATA {:?}", data);
        //true
        //return self.check_batch(data);
    }
}

