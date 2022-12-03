


use super::{tokenizer_holder::TokenizerHolder, tokenizer_config::{ TokenizerTask, TokenizerInternalConfig}};



pub struct BertTokenizer {
    pub tokenizer:TokenizerHolder, 
    pub mask:u32,
    pub cls:u32,
    pub sep:u32,
    pub pad:u32   
}

pub struct GptTokenizer {
    pub tokenizer:TokenizerHolder,
    pub eos:u32
}

pub struct T5Tokenizer {
    pub tokenizer:TokenizerHolder,
    pub eos:u32,
    pub pad:u32,  
    pub unk:u32,
    pub extra:Vec<u32>
}



pub enum TokenizerWrapper {
    Bert(BertTokenizer),
    Gpt(GptTokenizer),
    T5(T5Tokenizer),
}

impl TokenizerWrapper {

    pub fn get_extra_ids(&self) -> Vec<u32> {
        match self {
            TokenizerWrapper::T5(x) => x.extra.clone(),
            _ => todo!(),
        }
    }

    pub fn encode_mask(&mut self, data:String) -> Vec<u32> {

        match self {
            TokenizerWrapper::Bert(t) => {
                // Surround the sequence with the start and end tokens
                let mut ids = t.tokenizer.get_ids(data);
                ids.insert(0, t.cls);
                ids.insert(ids.len(), t.sep);
                ids.insert(ids.len(), t.sep);
                return ids;
            },
            TokenizerWrapper::Gpt(t) => {
                let mut ids = t.tokenizer.get_ids(data);
                // Surround the sequence with the start and end tokens
                ids.insert(0, t.eos);
                ids.insert(ids.len(), t.eos);
                return ids;
            },
            TokenizerWrapper::T5(t) => {
                let mut ids = t.tokenizer.get_ids(data);
                // Surround the sequence with the start and end tokens
                ids.insert(0, t.eos);
                ids.insert(ids.len(), t.eos);
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
        match self {
            TokenizerWrapper::Bert(x) => Some(x.mask),
            _ => None,
        }
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
                mask: holder.token_to_id("[MASK]").unwrap(), 
                cls:  holder.token_to_id("[CLS]").unwrap(), 
                sep:  holder.token_to_id("[SEP]").unwrap(), 
                pad:  holder.token_to_id("[PAD]").unwrap(),
                tokenizer: holder};
            return Some(TokenizerWrapper::Bert(wrapper));
        },
        TokenizerTask::Roberta => {
            let wrapper = BertTokenizer{  
                mask: holder.token_to_id("<mask>").unwrap(), 
                cls:  holder.token_to_id("<s>").unwrap(), 
                sep:  holder.token_to_id("</s>").unwrap(), 
                pad:  holder.token_to_id("<pad>").unwrap(),
                tokenizer: holder};
            return Some(TokenizerWrapper::Bert(wrapper));
        },
        TokenizerTask::T5 => {
            let mut extra = Vec::<u32>::with_capacity(100);
            for i in 0..100 {
                extra.push(holder.token_to_id(format!("<extra_id_{i}>").as_str()).unwrap().to_owned())
            }
            let wrapper = T5Tokenizer{  
                extra: extra,
                eos:  holder.token_to_id("</s>").unwrap().to_owned(), 
                unk:  holder.token_to_id("<unk>").unwrap().to_owned(), 
                pad:  holder.token_to_id("<pad>").unwrap().to_owned(),
                tokenizer: holder};
            return Some(TokenizerWrapper::T5(wrapper));
        },
        TokenizerTask::Gpt => {
            let wrapper = GptTokenizer{ 
                eos: holder.token_to_id("<|endoftext|>").unwrap().to_owned(), 
                tokenizer: holder};
            return Some(TokenizerWrapper::Gpt(wrapper));
        },
    }
    
}
