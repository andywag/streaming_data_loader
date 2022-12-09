
use serde::{Serialize, Deserialize, ser::SerializeStruct};
use crate::{batcher::BatchConfig, datasets::dataset_config::{DataSetConfig}, models::simple_label::Label};

use rand::prelude::*;
use rand_distr::StandardNormal;


#[derive(Debug, Clone, Deserialize)]
pub struct T5Data {
    pub input_ids:Vec<Vec<u32>>,
    pub attention_mask:Vec<Vec<u32>>,
    pub labels:Vec<Vec<i32>>,
    index:usize,
    pub remaining:Option<Vec<u32>>,

    batch_config:BatchConfig, 
    attention_base:Vec<u32>,
    extra_ids:Vec<u32>,

    avg_span_gap:f64,
    avg_span_size:f64
    
}

impl T5Data {
    pub fn new(batch_config:BatchConfig, dataset_config:DataSetConfig) -> Self{
        let sequence_length = batch_config.sequence_length;
        
        let (number_spans, mask_probability) = match dataset_config {
            DataSetConfig::T5 { number_spans, mask_probability } => (number_spans, mask_probability),
            _ => panic!("Operation Not Supported for T5!")
        };

        let avg_span_gap:f64 = (batch_config.sequence_length as f64/number_spans as f64)*(1.0-mask_probability);
        let avg_span_size:f64 = (batch_config.sequence_length as f64/number_spans as f64)*mask_probability;


        Self {
            input_ids: vec![vec![0;batch_config.sequence_length];batch_config.batch_size],
            attention_mask: vec![vec![1;batch_config.sequence_length];batch_config.batch_size],
            labels:vec![vec![-100;batch_config.sequence_length]; batch_config.batch_size],
            index:0, 
            remaining:None,

            batch_config:batch_config, 

            attention_base:vec![0;sequence_length as usize],
            extra_ids:vec![30000;100], // TODO : Fixme lazily put here
            avg_span_gap:avg_span_gap,
            avg_span_size:avg_span_size
        }
    }


    // TODO : Add Round and Proper Std scaling
    pub fn random_data_gap(&mut self) -> usize {
        let val: f64 = thread_rng().sample(StandardNormal);
        let distance = self.avg_span_gap - val;
        distance as usize
    }

    // TODO : Add Round and Proper Std Scaling
    pub fn random_data_size(&mut self) -> usize {
        let val: f64 = thread_rng().sample(StandardNormal);
        let distance = self.avg_span_size - val;
        std::cmp::max(distance as usize,1)
    }

    pub fn put_data(&mut self, ids:Vec<u32>, _label:Option<Label>) -> bool{

        //log::info!("Putting Data");
        let mut ip:usize = 0;
        while self.index < self.batch_config.batch_size {
            let mut lp = 0;
            let mut ap = 0;
            let mut pass = 0;
            while lp < self.batch_config.sequence_length {
                let mut data_gap = self.random_data_gap();
                data_gap = std::cmp::min(data_gap, self.batch_config.sequence_length - lp);
                data_gap = std::cmp::min(data_gap, ids.len() - ip);
                //log::info!("Hh {} {} {} {} {} ", self.index, ip, lp, pass, data_gap);

                if data_gap > 0 {
                    self.input_ids[self.index][lp..lp+data_gap].clone_from_slice(&ids[ip..ip+data_gap]);
                    lp += data_gap; ip += data_gap;
                }
                data_gap = self.random_data_size();
                data_gap = std::cmp::min(data_gap, self.batch_config.sequence_length - lp);
                data_gap = std::cmp::min(data_gap, ids.len() - ip);
                //log::info!("Ha {} {} {} {} {} {}", self.index, ip, lp, ap, pass, data_gap);

                if data_gap > 0 {
                    self.input_ids[self.index][lp] = self.extra_ids[pass]; // TODO : Mask Token
                    self.labels[self.index][ap] = self.extra_ids[pass] as i32; // TODO : Mask Token
                    for i in 0..data_gap {
                        self.labels[self.index][ap+i+1] = ids[ip + i] as i32;
                    }
                    lp += 1; ip += data_gap; ap += data_gap + 1;
                }
                if ids.len() <= ip {
                    self.attention_mask[self.index][lp..self.batch_config.sequence_length].copy_from_slice(&self.attention_base[0..self.batch_config.sequence_length-lp]);
                    self.labels[self.index][ap] = self.extra_ids[pass + 1] as i32;
                    self.index += 1;
                    return false;
                }
                pass += 1;
                //log::info!("H {} {} {} {} {} {} {}", ids.len(), self.index, ip, lp, ap, pass, data_gap);
            }
            self.index += 1;
        }
        self.remaining = Some(ids[ip..ids.len()].to_vec());
        self.done()
    }

    pub fn done(&self) -> bool{
        self.index == self.input_ids.len()
    }


}

impl Serialize for T5Data {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
            let mut state = serializer.serialize_struct("T5Data", 3)?;
            state.serialize_field("input_ids", &self.input_ids)?;
            state.serialize_field("attention_mask", &self.attention_mask)?;
            state.serialize_field("labels", &self.labels)?;
            state.end()
    }
}