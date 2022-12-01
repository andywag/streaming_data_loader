
use tokenizers::Tokenizer;
use std::thread;

use crate::{tokenizer_holder::TokenizerHolder, tasks::python::base_tokenizer::PythonTokenizer};


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



fn get_hugging_tokenizer(location:String) -> Option<Tokenizer> {
    let (tx,rx)= std::sync::mpsc::channel::<Tokenizer>();
    //let location_clone = location.clone();
    thread::spawn(move || {
        let base = Tokenizer::from_pretrained(location, None);
        let _ =tx.send(base.unwrap());
    });
    match rx.recv() {
        Ok(x) => {
            Some(x)
        },
        Err(e) => {
            log::error!("Couldn't Open Tokenizer {:?}", e);
            None
        },
    }    
}

/// Convenience Function to Get Tokenizer Uses thread to get around issues with tokio async
pub fn get_tokenizer(location:String, mode:String) -> Option<TokenizerWrapper> {
    
    let x = match mode.as_str() {
        "python" => TokenizerHolder::Python(PythonTokenizer::new(32768)),
        _ => {
            let tokenizer = get_hugging_tokenizer(location.clone());
            TokenizerHolder::HuggingFace(tokenizer.unwrap())
        }
    }; 

    if location.contains("roberta") {
        let wrapper = BertTokenizer{  
            mask: x.token_to_id("<mask>").unwrap(), 
            cls:  x.token_to_id("<s>").unwrap(), 
            sep:  x.token_to_id("</s>").unwrap(), 
            pad:  x.token_to_id("<pad>").unwrap(),
            tokenizer: x};
        return Some(TokenizerWrapper::Bert(wrapper));
    }
    else if location.contains("bert") {
        let wrapper = BertTokenizer{  
            mask: x.token_to_id("[MASK]").unwrap().to_owned(), 
            cls:  x.token_to_id("[CLS]").unwrap().to_owned(), 
            sep:  x.token_to_id("[SEP]").unwrap().to_owned(), 
            pad:  x.token_to_id("[PAD]").unwrap().to_owned(),
            tokenizer: x};
        return Some(TokenizerWrapper::Bert(wrapper));
    }
    else if location.contains("gpt") { 
        let wrapper = GptTokenizer{ 
            eos: x.token_to_id("<|endoftext|>").unwrap().to_owned(), 
            tokenizer: x};
        return Some(TokenizerWrapper::Gpt(wrapper));
    }
    else if location.contains("t5") { 
        let mut extra = Vec::<u32>::with_capacity(100);
        for i in 0..100 {
            extra.push(x.token_to_id(format!("<extra_id_{i}>").as_str()).unwrap().to_owned())
        }
        let wrapper = T5Tokenizer{  
            extra: extra,
            eos:  x.token_to_id("</s>").unwrap().to_owned(), 
            unk:  x.token_to_id("<unk>").unwrap().to_owned(), 
            pad:  x.token_to_id("<pad>").unwrap().to_owned(),
            tokenizer: x};
        return Some(TokenizerWrapper::T5(wrapper));
    }
    else {
        log::error!("Couldn't Find Wrapper for Tokenizer {:?}", location);
    }
    None
}
