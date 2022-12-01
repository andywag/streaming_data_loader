

use tokenizers::{Tokenizer};

use crate::tasks::python::base_tokenizer::PythonTokenizer;


pub enum TokenizerHolder {
    HuggingFace(Tokenizer),
    Python(PythonTokenizer)
}

impl TokenizerHolder {
    pub fn get_ids(&mut self, data:String) -> Vec<u32> {
        match self {
            TokenizerHolder::HuggingFace(x) => {
                let result = x.encode(data, true);
                result.unwrap().get_ids().to_vec()    
            }
            TokenizerHolder::Python(x) => {
                x.encode(data)
            },
        }
    }

    pub fn encode(&self, data:tokenizers::EncodeInput) -> Option<tokenizers::Encoding> {
        match self {
            TokenizerHolder::HuggingFace(x) => {
                Some(x.encode(data, true).unwrap()) 
            }
            TokenizerHolder::Python(_) => {
                None
            }
        }
    }

    pub fn token_to_id(&self, token:&str) -> Option<u32> {
        match self {
            TokenizerHolder::HuggingFace(x) => {
                x.token_to_id(token) 
            }
            // TODO : Need to add extra ids
            TokenizerHolder::Python(_) => {
                match token {
                    "<mask>" | "[MASK]" => Some(5),
                    "<pad>" | "[PAD]" => Some(0),
                    "<s>" | "[CLS]" => Some(1),
                    "</s>" => Some(2),
                    "<|endoftext|>" => Some(3),
                    "<unk" => Some(4),

                    _ => None
                }
            }
        }
    }

    

}