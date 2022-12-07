use std::cmp::min;

use serde::{Serialize, Deserialize, ser::SerializeStruct};

use crate::batcher::BatchConfig;

use super::SingleClassConfig;

#[derive(Debug, Clone, Deserialize)]
pub struct SingleClassData {
    pub input_ids:Vec<Vec<u32>>,
    pub attention_mask:Vec<Vec<u32>>,
    pub token_type_ids:Vec<Vec<u32>>,
    pub label:Vec<u32>,
    config:SingleClassConfig,
    batch_config:BatchConfig,

    index:usize
}

impl SingleClassData {
    pub fn new(config:&SingleClassConfig, batch_config:BatchConfig) -> Self{
        Self {
            input_ids: batch_config.create_vector(0),
            attention_mask: batch_config.create_vector(0),
            token_type_ids: batch_config.create_vector(0),
            label: batch_config.create_vector_1d(0),
            config:config.clone(),
            batch_config:batch_config,

            index:0
        }
    }

    pub fn new_data(&mut self) -> Self {
        Self::new(&self.config, self.batch_config.clone())
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

#[derive(Debug, Clone)]
pub struct SingleClassTransport {
    pub text:String,
    pub label:u32,
}

impl Serialize for SingleClassData {
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