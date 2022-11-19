
use tokenizers::Tokenizer;
use std::thread;

pub struct BertTokenizer {
    pub tokenizer:Tokenizer, 
    pub mask:u32,
    pub cls:u32,
    pub sep:u32,
    pub pad:u32   
}

pub struct RobertaTokenizer {
    pub tokenizer:Tokenizer,
    pub mask:u32,
    pub cls:u32,
    pub sep:u32,
    pub pad:u32   
}

pub struct GptTokenizer {
    pub tokenizer:Tokenizer,
    pub eos:u32
}



impl RobertaTokenizer {
    pub fn new(tokenizer:Tokenizer) -> Self {
        let cls = tokenizer.token_to_id("<s>").unwrap().to_owned();
        let sep = tokenizer.token_to_id("</s>").unwrap().to_owned();
        let mask = tokenizer.token_to_id("<mask>").unwrap().to_owned();
        let pad = tokenizer.token_to_id("<pad>").unwrap().to_owned();

        Self {
            tokenizer: tokenizer,
            cls:cls,
            sep:sep,
            mask:mask,
            pad:pad
        }
    }
}

impl BertTokenizer {
    pub fn new(tokenizer:Tokenizer) -> Self {

        let cls = tokenizer.token_to_id("[CLS]").unwrap().to_owned();
        let sep = tokenizer.token_to_id("[SEP]").unwrap().to_owned();
        let mask = tokenizer.token_to_id("[MASK]").unwrap().to_owned();
        let pad = tokenizer.token_to_id("[PAD]").unwrap().to_owned();

        Self {
            tokenizer: tokenizer,
            cls:cls,
            sep:sep,
            mask:mask,
            pad:pad
        }
    }
}

impl GptTokenizer {
    pub fn new(tokenizer:Tokenizer) -> Self {
        let eos_token = tokenizer.token_to_id("<|endoftext|>").unwrap().to_owned();

        Self {
            tokenizer: tokenizer,
            eos:eos_token,
        }
    }
}

pub enum TokenizerWrapper {
    Bert(BertTokenizer),
    Roberta(RobertaTokenizer),
    Gpt(GptTokenizer)
}

impl TokenizerWrapper {

    pub fn get_tokenizer(&self) -> &Tokenizer {

        match self {
            TokenizerWrapper::Bert(t) => {
                &t.tokenizer
            },
            TokenizerWrapper::Roberta(t) => {
                &t.tokenizer
            },
            TokenizerWrapper::Gpt(t) => {
                &t.tokenizer
            },
        }
    }

    pub fn encode_mask(&self, data:String) -> Vec<u32> {

        match self {
            TokenizerWrapper::Bert(t) => {
                // Surround the sequence with the start and end tokens
                let result = t.tokenizer.encode(data, true).unwrap();
                let mut ids:Vec<u32> = result.get_ids().to_vec();

                ids.insert(0, t.cls);
                ids.insert(ids.len(), t.sep);
                ids.insert(ids.len(), t.sep);
                return ids;
            },
            TokenizerWrapper::Roberta(t) => {
                let result = t.tokenizer.encode(data, true).unwrap();
                let mut ids:Vec<u32> = result.get_ids().to_vec();

                ids.insert(0, t.cls);
                ids.insert(ids.len(), t.sep);
                ids.insert(ids.len(), t.sep);
                return ids;
            },
            TokenizerWrapper::Gpt(t) => {
                let result = t.tokenizer.encode(data, true).unwrap();
                let mut ids:Vec<u32> = result.get_ids().to_vec();

                // Surround the sequence with the start and end tokens
                ids.insert(0, t.eos);
                ids.insert(ids.len(), t.eos);
                return ids;
            },
        }
    }


    pub fn encode(&self, data:tokenizers::EncodeInput) -> tokenizers::Encoding {

        let result = match self {
            TokenizerWrapper::Bert(t) => {
                t.tokenizer.encode(data, true)
            },
            TokenizerWrapper::Roberta(t) => {
                t.tokenizer.encode(data, true)
            },
            TokenizerWrapper::Gpt(t) => {
                t.tokenizer.encode(data, true)
            },
        };
        result.unwrap()
    }

    pub fn mask_token(&self) -> Option<u32> {
        match self {
            TokenizerWrapper::Bert(x) => Some(x.mask),
            TokenizerWrapper::Roberta(x) => Some(x.mask),
            _ => None,
        }
    }

}


/// Convenience Function to Get Tokenizer Uses thread to get around issues with tokio async
pub fn get_tokenizer(location:String) -> Option<TokenizerWrapper> {
    let (tx,rx)= std::sync::mpsc::channel::<Tokenizer>();
    let location_clone = location.clone();
    thread::spawn(move || {
        let base = Tokenizer::from_pretrained(location, None);
        let _ =tx.send(base.unwrap());
    });
    match rx.recv() {
        Ok(x) => {
            if location_clone.contains("roberta") {
                return Some(TokenizerWrapper::Roberta(RobertaTokenizer::new(x)))
            }
            else if location_clone.contains("bert") {
                return Some(TokenizerWrapper::Bert(BertTokenizer::new(x)))
            }
            else if location_clone.contains("gpt") { 
                return Some(TokenizerWrapper::Gpt(GptTokenizer::new(x)))
            }
            else {
                log::error!("Couldn't Find Wrapper for Tokenizer {:?}", location_clone);
            }
        },
        Err(e) => {
            log::error!("Couldn't Open Tokenizer {:?}", e);
        },
    }
    None
    
}