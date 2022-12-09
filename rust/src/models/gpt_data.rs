
use serde::{Serialize, Deserialize, ser::SerializeStruct};

use crate::{batcher::BatchConfig, models::simple_label::Label, datasets::dataset_config::DataSetConfig};

#[derive(Debug, Clone, Deserialize)]
pub struct GptData {
    pub input_ids:Vec<Vec<u32>>,
    pub attention_mask:Vec<Vec<u32>>,
    pub labels:Vec<Vec<i32>>,
    pub index:usize,

    batch_config:BatchConfig,
}

impl GptData {
    pub fn new(batch_config:BatchConfig, _dataset_config:DataSetConfig) -> Self{
        Self {
            input_ids: batch_config.create_vector(0),
            attention_mask: batch_config.create_vector(1),
            labels:batch_config.create_vector(-100),
            index:0,
            
            batch_config:batch_config,
            
        }
    }

    pub fn put_data(&mut self, ids:Vec<u32>, _label:Option<Label>) -> bool{
        self.input_ids[self.index][0..ids.len() as usize].clone_from_slice(&ids);
        let labels:Vec<i32> = self.input_ids[self.index].clone().into_iter().map(|e| e as i32).collect();
        self.labels[self.index][0..labels.len() as usize].clone_from_slice(&labels);
        if ids.len() < self.batch_config.sequence_length {
            let s = self.batch_config.sequence_length as usize;
            let l = ids.len() as usize;
            for x in s-l..s {
                self.labels[self.index][x] = -100;
                self.attention_mask[self.index][x] = 0;
            }
            
        }
        self.index += 1;
        self.done()
        
    }

    pub fn done(&self) -> bool{
        self.index == self.input_ids.len()
    }

}

impl Serialize for GptData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
            let mut state = serializer.serialize_struct("GptData", 3)?;
            state.serialize_field("input_ids", &self.input_ids)?;
            state.serialize_field("attention_mask", &self.attention_mask)?;
            state.serialize_field("labels", &self.labels)?;
            state.end()
    }
}