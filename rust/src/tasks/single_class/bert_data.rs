use std::cmp::min;

use serde::{Serialize, Deserialize, ser::SerializeStruct};

use crate::batcher::BatchConfig;

use super::single_data::SingleClassData;
use core::fmt::Debug;

trait Takes<T> {
    fn take(&mut self, _:T) -> bool;
}

#[derive(Debug, Clone, Deserialize)]
pub struct BertData<T:Serialize + Debug + Clone> {
    pub input_ids:Vec<Vec<u32>>,
    pub attention_mask:Vec<Vec<u32>>,
    pub token_type_ids:Vec<Vec<u32>>,
    pub label:Option<Vec<T>>,

    index:usize
}

impl <T:Serialize + Debug + Clone>BertData<T> {
    pub fn new(batch_config:BatchConfig) -> Self{
        Self {
            input_ids: batch_config.create_vector(0),
            attention_mask: batch_config.create_vector(0),
            token_type_ids: batch_config.create_vector(0),
            label: None,

            index:0
        }
    } 

    pub fn put_data(&mut self, result:&tokenizers::Encoding, label:u32) -> bool {

        let ids = result.get_ids();
        let length = min(ids.len(), self.batch_config.sequence_length);
        self.input_ids[self.index][0..length].clone_from_slice(&ids[0..length]);
        self.token_type_ids[self.index][0..length].clone_from_slice(&result.get_type_ids()[0..length]);
        self.attention_mask[self.index][0..length].clone_from_slice(&result.get_attention_mask()[0..length]);
        self.label[self.index] = label;

        self.index += 1;
        self.done()
    }

    pub fn done(&self) -> bool {
        self.index == self.batch_config.batch_size
    }

}

impl Takes<SingleClassTransport> for SingleClassData {
    fn take(&mut self, transport: SingleClassTransport, tokenizer:TokenizerWrapper) -> bool {
        true
    }
}

#[derive(Debug, Clone)]
pub struct SingleClassTransport {
    pub text:String,
    pub label:u32,
}

impl <T:Serialize+Clone+Debug>Serialize for BertData<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
            let mut state = serializer.serialize_struct("SingleClassData", 4)?;
            state.serialize_field("input_ids", &self.input_ids)?;
            state.serialize_field("attention_mask", &self.attention_mask)?;
            state.serialize_field("token_type_ids", &self.token_type_ids)?;
            state.serialize_field("label", &self.label)?;
            state.end()
    }
}