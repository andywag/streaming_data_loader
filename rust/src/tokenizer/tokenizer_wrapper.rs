


use serde::Deserialize;

use crate::models::simple_transport::SimpleData;

use super::{tokenizer_holder::TokenizerHolder, tokenizer_config::{ TokenizerTask, TokenizerInternalConfig}};

#[derive(Clone, Debug, Deserialize)]
pub struct TokenizerInfo {
    pub cls:u32,
    pub sep:u32,
    pub pad:u32,
    pub mask:u32,
    pub unk:u32,
    pub extra:Vec<u32>,
    pub eos:u32
}

pub struct BertTokenizer {
    pub tokenizer:TokenizerHolder, 
}

pub struct GptTokenizer {
    pub tokenizer:TokenizerHolder,
}

pub struct T5Tokenizer {
    pub tokenizer:TokenizerHolder,
}



pub enum TokenizerWrapper {
    Bert(BertTokenizer),
    Gpt(GptTokenizer),
    T5(T5Tokenizer)
}

impl TokenizerWrapper {



    pub fn get_tokenizer_info(&self) -> TokenizerInfo {
        let extra = vec![];
        let eos = 0;
        let cls = 0;
        let sep = 0;
        let pad = 0;
        let mask = 0;
        let unk = 0;
        match self {
            TokenizerWrapper::Bert(tokenizer) => {
                TokenizerInfo {
                    cls: tokenizer.tokenizer.token_to_id("[CLS]").unwrap(),
                    sep: tokenizer.tokenizer.token_to_id("[SEP]").unwrap(),
                    pad: tokenizer.tokenizer.token_to_id("[PAD]").unwrap(),
                    mask: tokenizer.tokenizer.token_to_id("[MASK]").unwrap(),
                    extra,
                    eos,
                    unk,
                }
            },
            TokenizerWrapper::Gpt(tokenizer) => {
                TokenizerInfo {
                    cls,
                    sep,
                    pad,
                    mask,
                    extra,
                    eos:tokenizer.tokenizer.token_to_id("<|endoftext|>").unwrap(),
                    unk,
                }
            },
            TokenizerWrapper::T5(tokenizer) => {
                let mut extra = Vec::<u32>::with_capacity(100);
                for x in 0..100 {
                    extra.push(tokenizer.tokenizer.token_to_id(format!("<extra_id_{x}>").as_str()).unwrap());
                }
                TokenizerInfo {
                    cls,
                    sep,
                    pad: tokenizer.tokenizer.token_to_id("<pad>").unwrap(),
                    mask,
                    extra,
                    eos: tokenizer.tokenizer.token_to_id("</s>").unwrap(),
                    unk: tokenizer.tokenizer.token_to_id("<unk>").unwrap()
                }
            },
        }
    }

    pub fn get_extra_ids(&self) -> Vec<u32> {
        match self {
            TokenizerWrapper::T5(_x) => self.get_tokenizer_info().extra.clone(),
            _ => todo!(),
        }
    }

    pub fn encode_simple(&mut self, data:SimpleData) -> (Vec<u32>, Option<Vec<u32>>) {
        let result = self.encode_mask(data.text);
        let alt_result:Option<Vec<u32>> = data.alt_text.map(|s|self.encode_mask(s));
        (result,alt_result)
    }

    pub fn encode_mask(&mut self, data:String) -> Vec<u32> {
        let info = self.get_tokenizer_info();
        match self {
            TokenizerWrapper::Bert(t) => {
                // Surround the sequence with the start and end tokens
                let mut ids = t.tokenizer.get_ids(data);
                ids.insert(0, info.cls);
                ids.insert(ids.len(), info.sep);
                ids.insert(ids.len(), info.sep);
                return ids;
            },
            TokenizerWrapper::Gpt(t) => {
                let mut ids = t.tokenizer.get_ids(data);
                // Surround the sequence with the start and end tokens
                ids.insert(0, info.eos);
                ids.insert(ids.len(), info.eos);
                return ids;
            },
            TokenizerWrapper::T5(t) => {
                let mut ids = t.tokenizer.get_ids(data);
                // Surround the sequence with the start and end tokens
                ids.insert(0, info.eos);
                ids.insert(ids.len(), info.eos);
                return ids;
            },
           
        }
    }


    pub fn encode(&self, data:tokenizers::EncodeInput) -> tokenizers::Encoding {

        match self {
            TokenizerWrapper::Bert(t) => {
                t.tokenizer.encode(data).unwrap()
            },
            TokenizerWrapper::Gpt(t) => {
                t.tokenizer.encode(data).unwrap()
            },
            TokenizerWrapper::T5(t) => {
                t.tokenizer.encode(data).unwrap()
            },
        }
    }

    pub fn mask_token(&self) -> Option<u32> {
        Some(self.get_tokenizer_info().mask)
    }

}




/// Convenience Function to Get Tokenizer Uses thread to get around issues with tokio async
pub fn get_tokenizer(config:TokenizerInternalConfig) -> Option<TokenizerWrapper> {
    
    //let mode = config.mode.unwrap_or("test".to_string());
    //let location = config.name;

    let holder = super::tokenizer_holder::create_tokenizer_holder(config.typ);
    match config.task {
        TokenizerTask::Bert =>  {
            let wrapper = BertTokenizer{  
                tokenizer: holder};
            return Some(TokenizerWrapper::Bert(wrapper));
        },
        TokenizerTask::Roberta => {
            let wrapper = BertTokenizer{  
                tokenizer: holder};
            return Some(TokenizerWrapper::Bert(wrapper));
        },
        TokenizerTask::T5 => {
            let wrapper = T5Tokenizer{tokenizer: holder};
            return Some(TokenizerWrapper::T5(wrapper));
        },
        TokenizerTask::Gpt => {
            let wrapper = GptTokenizer{tokenizer: holder};
            return Some(TokenizerWrapper::Gpt(wrapper));
        },
    }
    
}
