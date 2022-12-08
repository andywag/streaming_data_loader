

use serde::{Serialize, Deserialize, ser::SerializeStruct};
use rand::prelude::SliceRandom;
use rand::thread_rng;

use crate::{batcher::BatchConfig, tokenizer::tokenizer_data::TokenizedData};

use super::{config::PythonConfig};

#[derive(Debug, Clone, Deserialize)]
pub struct PythonData {
    pub input_ids:Vec<Vec<u32>>,
    pub position_ids:Vec<Vec<u32>>,
    pub attention_mask:Vec<Vec<Vec<u32>>>,
    pub labels:Vec<Vec<i32>>,
    index:usize,

    pub config:PythonConfig,
    batch_config:BatchConfig,

    masked_length:usize,
    mask:u32
}

impl PythonData {
    pub fn new(config:PythonConfig, batch_config:BatchConfig, mask:u32) -> Self{
        let mask_length = config.mask_length;
        let number_context_layers = config.context_shape.len();

        Self {
            input_ids: batch_config.create_vector(0),
            position_ids: batch_config.create_vector(0),
            attention_mask: vec![vec![vec![255;batch_config.sequence_length];number_context_layers]; batch_config.batch_size],
            labels:vec![vec![-100;batch_config.sequence_length]; batch_config.batch_size],
            index:0, 

            config:config,
            batch_config:batch_config,
            masked_length:mask_length,
            mask:mask
        }
    }

    pub fn new_data(&mut self) -> Self {
        PythonData::new(self.config.clone(), self.batch_config.clone(), self.mask)
    }

    pub fn mask_batch(&mut self) {

        //self.position_base.shuffle(&mut thread_rng());
        let mut position_base:Vec<u32> = (0..self.batch_config.sequence_length as u32).collect();
        position_base.shuffle(&mut thread_rng());

        for x in 0..self.masked_length as usize {
            if self.input_ids[self.index][position_base[x] as usize] != 0 {
                self.labels[self.index][position_base[x] as usize] = self.input_ids[self.index][position_base[x] as usize] as i32;
                self.input_ids[self.index][position_base[x] as usize] = self.mask;       
            }
        }
        
    }

    pub fn put_data(&mut self, data:TokenizedData) -> bool{
        // TODO : Fix this condition by limiting data size
        if data.ids.len() < 64 {
            return false;
        }
        let l = std::cmp::min(self.batch_config.sequence_length, data.ids.len());
        //log::info!("Data {} {} {} {}", data.ids.len(), l, &data.ids[0..l].len(), self.input_ids[self.index][0..l as usize].len());
        let ids = &data.ids[0..l];
        let positions = &data.positions[0..l];

        self.input_ids[self.index][0..l].clone_from_slice(ids);
        self.position_ids[self.index][0..l as usize].clone_from_slice(positions);
        
        //log::info!("Attention {:?}", data.attention_mask);
        for x in 0..self.config.context_shape.len() {
            let attention = &data.attention_mask[x];
            let attention = &attention[0..l];
            //log::info!("Size {:?}", attention[0..l].len());
            self.attention_mask[self.index][x][0..l].clone_from_slice(attention);
        }


        //log::info!("Data {:?}", self.input_ids);
        //log::info!("Positions {:?}", self.position_ids);
        //log::info!("Attention {:?}", self.attention_mask);

        self.mask_batch();
        self.index += 1;
        self.done()
    }

    pub fn done(&self) -> bool{
        self.index == self.input_ids.len()
       
    }


}

impl Serialize for PythonData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
            let mut state = serializer.serialize_struct("PythonData", 3)?;
            state.serialize_field("input_ids", &self.input_ids)?;
            state.serialize_field("position_ids", &self.position_ids)?;
            state.serialize_field("attention_mask", &self.attention_mask)?;
            state.serialize_field("labels", &self.labels)?;
            state.end()
    }
}