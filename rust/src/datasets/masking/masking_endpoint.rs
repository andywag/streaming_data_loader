
use tokenizers::Tokenizer;

use crate::{datasets::masking::{masking_config::MaskingConfig, masked_data::MaskedData}, utils, transport::ZmqChannel, endpoint::EndPoint};

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
        // Compare only the first batch of data based on a known dataset
        for _x in 0..data.input_ids.len() as usize {
            for _y in 0..data.masked_lm_labels.len() as usize {
                //data.input_ids[x][data.masked_lm_positions[x][y] as usize] = data.masked_lm_labels[x][y];
            }
        }
        true
        /*let check = match compare_location {
            Some(path) => {
                utils::compare_data(path, data.input_ids, 256)    
            },
            None => false,
        };*/
        //println!("Matched {}", check);
    }
}

impl EndPoint<MaskedData> for MaskingEndpoint {
    fn receive(&mut self, data:MaskedData) -> bool {
        return self.check_batch(data);
    }
}


pub async fn receiver(config_in:&MaskingConfig, 
    mut rx:tokio::sync::mpsc::Receiver<ZmqChannel<MaskedData>>,
    compare_location:Option<String>
) -> bool {
    let config = config_in.clone();
    let data_full = rx.recv().await.unwrap();

    let mut data:MaskedData;
    match data_full {
        ZmqChannel::Complete => {
            println!("First Batch Required");
            data = MaskedData::new(1, 1, 1);
        },
        ZmqChannel::Data(x) => {
            data = x;
        },
    }

    // Compare only the first batch of data based on a known dataset
    for x in 0..config.batch_size as usize {
        for y in 0..config.mask_length as usize {
            data.input_ids[x][data.masked_lm_positions[x][y] as usize] = data.masked_lm_labels[x][y];
        }
    }
    let check = match compare_location {
        Some(path) => {
            utils::compare_data(path, data.input_ids, 256)    
        },
        None => false,
    };
    println!("Matched {}", check);
    
    // Wait for the rest of the inputs to flush out to exit
    loop {
        let result = rx.recv().await.unwrap();
        match result {
            ZmqChannel::Complete => {
                println!("Done Receiver");
                return check;
            },
            ZmqChannel::Data(_) => {},
        }
    }
    //println!("Test Receiver Finished");

    //return check;
}
