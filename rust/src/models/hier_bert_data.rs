

use serde::{Serialize, Deserialize, ser::SerializeStruct};
use rand::prelude::SliceRandom;
use rand::thread_rng;

use crate::{batcher::BatchConfig, tokenizer::tokenizer_data::TokenizedData, models::simple_label::Label, datasets::dataset_config::DataSetConfig};
use rand::prelude::*;
use rand_distr::StandardNormal;

#[derive(Debug, Clone, Deserialize)]
pub struct BertHierData {
    pub input_ids:Vec<Vec<u32>>,
    pub position_ids:Vec<Vec<u32>>,
    pub attention_mask:Vec<Vec<Vec<u32>>>,
    pub label:Vec<Label>,
    index:usize,

    batch_config:BatchConfig,
    dataset_config:DataSetConfig,
}

impl BertHierData {
    pub fn new(batch_config:BatchConfig, dataset_config:DataSetConfig, _mask:u32) -> Self{
        let context_size = match dataset_config.clone() {
            DataSetConfig::MaskHier { mask_length:_, context_size, front:_ } => context_size,
            //DataSetConfig::SpanHier { avg_span_gap:_, avg_span_size:_, context_size, extra_ids:_ } => context_size,
            _ => panic!("Data Hierarchichal Task Required"),
        };
        let number_context_layers = context_size;

        Self {
            input_ids: batch_config.create_vector(0),
            position_ids: batch_config.create_vector(0),
            attention_mask: vec![vec![vec![255;batch_config.sequence_length];number_context_layers]; batch_config.batch_size],
            label:Vec::with_capacity(batch_config.batch_size),
            index:0, 

            batch_config,
            dataset_config:dataset_config,
        }
    }

    /// Simple Masking of the Data 
    pub fn mask_batch(&mut self, masked_length:usize, mask:u32) {
        let mut position_base:Vec<u32> = (0..self.batch_config.sequence_length as u32).collect();
        position_base.shuffle(&mut thread_rng());
        let mut new_labels = vec![-100;self.batch_config.sequence_length];

        for x in 0..masked_length as usize {
            if self.input_ids[self.index][position_base[x] as usize] != 0 {
                new_labels[position_base[x] as usize] = self.input_ids[self.index][position_base[x] as usize] as i32;
                self.input_ids[self.index][position_base[x] as usize] = mask;       
            }
        }
        self.label.push(new_labels.into());
    }

    /// Masking of the Data and converted to positions and labels
    pub fn mask_batch_front(&mut self, mask_length:usize, mask:u32) {
        let mut position_base:Vec<u32> = (0..self.batch_config.sequence_length as u32).collect();
        position_base.shuffle(&mut thread_rng());
        
        let mut masked_lm_labels = vec![-100;mask_length];
        let mut masked_lm_positions:Vec<u32> = vec![0;mask_length];

        let mut current = 0;
        for x in 0..mask_length as usize {
            if self.input_ids[self.index][position_base[x] as usize] != 0 {
                masked_lm_labels[current] = self.input_ids[self.index][position_base[x] as usize] as i32;
                masked_lm_positions[current] = position_base[x]/4; 
                self.input_ids[self.index][position_base[x] as usize] = mask;   
                current += 1;
            }
        }
        self.label.push((masked_lm_positions, masked_lm_labels).into());
    }

    // Create Span Labels 
    pub fn create_span(&mut self, data:TokenizedData, avg_span_gap:f64, avg_span_size:f64, extra_ids:&Vec<u32>, context_size:usize) {

        fn random_data_gap(avg_span_gap:f64) -> usize {
            let val: f64 = thread_rng().sample(StandardNormal);
            let distance = avg_span_gap - val;
            distance as usize
        }

        pub fn random_data_size(avg_span_size:f64) -> usize {
            let val: f64 = thread_rng().sample(StandardNormal);
            let distance = avg_span_size - val;
            std::cmp::max(distance as usize,1)
        }

        let mut ip:usize = 0;
        let mut lp = 0;
        let mut ap = 0;
        let mut pass = 0;

        let mut new_labels = vec![-100;self.batch_config.sequence_length];
        while lp < self.batch_config.sequence_length {
            let mut data_gap = random_data_gap(avg_span_gap);
            data_gap = std::cmp::min(data_gap, self.batch_config.sequence_length - lp);
            data_gap = std::cmp::min(data_gap, data.ids.len() - ip);
            //log::info!("Hh {} {} {} {} {} ", self.index, ip, lp, pass, data_gap);

            if data_gap > 0 {
                self.input_ids[self.index][lp..lp+data_gap].clone_from_slice(&data.ids[ip..ip+data_gap]);
                self.position_ids[self.index][lp..lp+data_gap].clone_from_slice(&data.positions[ip..ip+data_gap]);
                for x in 0..context_size {
                    self.attention_mask[self.index][x][lp..lp+data_gap].clone_from_slice(&data.attention_mask[x][ip..ip+data_gap]);
                }
                lp += data_gap; ip += data_gap;
            }
            data_gap = random_data_size(avg_span_size);
            data_gap = std::cmp::min(data_gap, self.batch_config.sequence_length - lp);
            data_gap = std::cmp::min(data_gap, data.ids.len() - ip);
                //log::info!("Ha {} {} {} {} {} {}", self.index, ip, lp, ap, pass, data_gap);

            if data_gap > 0 {
                self.input_ids[self.index][lp] = extra_ids[pass]; // TODO : Mask Token
                self.position_ids[self.index][lp] = data.positions[lp];
                for x in 0..context_size {
                    self.attention_mask[self.index][x][lp] = data.attention_mask[x][ip];
                }
                new_labels[ap] = (extra_ids[pass] as i32).into(); // TODO : Mask Token
                for i in 0..data_gap {
                    new_labels[ap+i+1] = (data.ids[ip + i] as i32).into();
                }
                lp += 1; ip += data_gap; ap += data_gap + 1;
            }
            if data.ids.len() <= ip {           
                new_labels[ap] = (extra_ids[pass + 1] as i32).into();
                self.index += 1;
            }
            pass += 1;
                //log::info!("H {} {} {} {} {} {} {}", ids.len(), self.index, ip, lp, ap, pass, data_gap);
        }
        self.label.push(new_labels.into());
    }

    pub fn put_data(&mut self, data:TokenizedData, _label:Option<Label>) -> bool{
        // TODO : Fix this condition by limiting data size
        if data.ids.len() < 64 {
            return false;
        }
        //let data_config_clone = self.dataset_config.clone();

        let l = std::cmp::min(self.batch_config.sequence_length, data.ids.len());
        let ids = &data.ids[0..l];
        let positions = &data.positions[0..l];
    
        self.input_ids[self.index][0..l].clone_from_slice(ids);
        self.position_ids[self.index][0..l as usize].clone_from_slice(positions);
            
        match &self.dataset_config {
    
            DataSetConfig::MaskHier { mask_length, context_size, front:true } => {
                for x in 0..context_size.to_owned() {
                    let attention = &data.attention_mask[x];
                    let attention = &attention[0..l];
                    self.attention_mask[self.index][x][0..l].clone_from_slice(attention);
                }
                self.mask_batch_front(mask_length.to_owned(), 5);
            },
            DataSetConfig::MaskHier { mask_length, context_size, front:false } => {
                for x in 0..context_size.to_owned() {
                    let attention = &data.attention_mask[x];
                    let attention = &attention[0..l];
                    self.attention_mask[self.index][x][0..l].clone_from_slice(attention);
                }
                self.mask_batch(mask_length.to_owned(), 5);
            },
            _ => panic!("Only Python Configuration Supported")
        }
    
        self.index += 1;
        self.done()
    }

    pub fn done(&self) -> bool{
        self.index == self.input_ids.len()
       
    }


}

impl Serialize for BertHierData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
            let mut state = serializer.serialize_struct("BertHierData
        ", 3)?;
            state.serialize_field("input_ids", &self.input_ids)?;
            state.serialize_field("position_ids", &self.position_ids)?;
            state.serialize_field("attention_mask", &self.attention_mask)?;
            match self.dataset_config {
                DataSetConfig::Mask { mask_length:_, mask:_ } => {
                    let data:Vec<Vec<i32>> = self.label.clone().into_iter().map(|s|s.get_vec_i32().unwrap()).collect();
                    state.serialize_field("labels", &data)?;
                },
                DataSetConfig::MaskHier { mask_length: _, context_size : _ , front: false} => {
                    let data:Vec<Vec<i32>> = self.label.clone().into_iter().map(|s|s.get_vec_i32().unwrap()).collect();
                    state.serialize_field("labels", &data)?;
                },
                DataSetConfig::MaskHier { mask_length: _, context_size : _ , front: true} => {
                    // TODO : Convert to unzip
                    let positions:Vec<Vec<u32>> = self.label.clone().into_iter().map(|s|s.get_masked_position().unwrap()).collect();
                    let labels:Vec<Vec<i32>> = self.label.clone().into_iter().map(|s|s.get_masked_label().unwrap()).collect();
                    state.serialize_field("masked_lm_positions", &positions)?;
                    state.serialize_field("masked_lm_labels", &labels)?;
                },
                _ => todo!()
            }
            state.end()
    }
}