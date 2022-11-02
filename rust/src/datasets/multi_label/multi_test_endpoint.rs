use tokenizers::Tokenizer;

use crate::{ utils, endpoint::EndPoint};

use super::{MultiConfig, multi_data::MultiData};



pub struct MultiTestEndpoint {
    _tokenizer:Tokenizer
}

impl MultiTestEndpoint {
    pub fn new(config:MultiConfig) -> Self {
        let tokenizer = utils::get_tokenizer(config.tokenizer_name.to_owned());
        Self {
            _tokenizer:tokenizer
        }
    }
}

impl EndPoint<MultiData> for MultiTestEndpoint {
    fn receive(&mut self, _data:MultiData) -> bool {
        return true;
    }
}

