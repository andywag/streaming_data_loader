

use crate::{transport::test_endpoint::EndPoint, datasets::DataSet, config::TrainingConfig};

use super::gpt_data::GptData;

pub struct Gpt2Endpoint {
    //pub tokenizer:Tokenizer
}

impl Gpt2Endpoint {
    pub fn new(_config:TrainingConfig) -> Self {
        //let tokenizer = utils::get_tokenizer(config.tokenizer_name.to_owned());
        Self {
            //tokenizer:tokenizer
        }
    }

    // TODO : Put in a valid check for the data. Disabled due to use of file match which was inconvenient
    // TODO : Make data input mutable to allow checks
    pub fn check_batch(&self, data:GptData) -> bool {
        //let mut real_data = data.input_ids.clone();
        //log::info!("Here I am to save teh day {:?}", data);
        // Compare only the first batch of data based on a known dataset
        for x in 0..data.input_ids.len() as usize {
            for y in 0..data.input_ids.len() as usize {
                if data.attention_mask[x][y] == 1 && data.input_ids[x][y] as i32 != data.labels[x][y] {
                    return false
                }
                else if data.attention_mask[x][y] == 0 && data.labels[x][y] != -100 {
                    return false
                }
            }
        }
        true
    }
}

impl EndPoint<DataSet> for Gpt2Endpoint {
    fn receive(&mut self, data:DataSet) -> bool {
        match data {
            DataSet::Gpt2(x) => self.check_batch(x),
            _ => {log::error!("Test Endpoint Only Supported for GPT2"); false}

        }
       
    }
}

