
use serde::{Serialize, Deserialize, ser::SerializeStruct};

use crate::{batcher::BatchConfig, tokenizer::tokenizer_wrapper::TokenizerWrapper, models::simple_label::{Label}, datasets::dataset_config::DataSetConfig};
use rand::{thread_rng, seq::SliceRandom};

use core::fmt::Debug;

trait Takes<T> {
    fn take(&mut self, _:T, tokenizer:&mut TokenizerWrapper) -> bool;
}

#[derive(Debug, Clone, Deserialize)]
pub struct BertData {
    pub input_ids:Vec<Vec<u32>>,
    pub attention_mask:Vec<Vec<u32>>,
    pub token_type_ids:Vec<Vec<u32>>,
    pub label:Vec<Label>,

    batch_config:BatchConfig,
    dataset_config:DataSetConfig,
    index:usize
}


impl BertData {
    pub fn new(batch_config:BatchConfig, dataset_config:DataSetConfig) -> Self{
        Self {
            input_ids: batch_config.create_vector(0),
            attention_mask: batch_config.create_vector(1),
            token_type_ids: batch_config.create_vector(0),
            label: Vec::with_capacity(batch_config.batch_size),

            dataset_config,
            batch_config,
            index:0
        }
    } 

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

    pub fn put_data(&mut self, ids:Vec<u32>, label:Option<Label>) -> bool{
        let l = std::cmp::min(self.batch_config.sequence_length, ids.len());
        self.input_ids[self.index][0..l as usize].clone_from_slice(&ids[0..l]);
        if ids.len() < self.batch_config.sequence_length {
            let s = self.batch_config.sequence_length;
            for x in s-ids.len()..s {
                self.attention_mask[self.index][x] = 0;
            }         
        }
        match self.dataset_config {
            
            DataSetConfig::MultiLabel { number_labels} => {
                match label.unwrap() {                    
                    Label::Multi(indices) => {
                        let mut new_labels = vec![0.0;number_labels];
                        for x in indices {
                            new_labels[x as usize] = 1.0;
                        }
                        self.label.push(new_labels.into());
                    },
                    _ => panic!("Label Type Not Supported")
                }
               
            },
            DataSetConfig::SingleClass => {
                label.map(|s| self.label.push(s));
            },
            DataSetConfig::Mask { mask_length, mask } => {
                self.mask_batch(mask_length, mask);
            }
            _ => todo!(),
        };
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

impl Serialize for BertData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
            let number_labels = match self.dataset_config {
                DataSetConfig::Squad => 5,
                _ => 4
            };

            let mut state = serializer.serialize_struct("SingleClassData", number_labels)?;
            state.serialize_field("input_ids", &self.input_ids)?;
            state.serialize_field("attention_mask", &self.attention_mask)?;
            state.serialize_field("token_type_ids", &self.token_type_ids)?;
            match self.dataset_config {
                DataSetConfig::SingleClass => {
                    let data:Vec<u32> = self.label.clone().into_iter().map(|s|s.get_single().unwrap()).collect();
                    state.serialize_field("label", &data)?;
                },
                DataSetConfig::MultiLabel { number_labels: _  } => {
                    let data:Vec<Vec<f32>> = self.label.clone().into_iter().map(|s|s.get_multi_f32().unwrap()).collect();
                    state.serialize_field("labels", &data)?;
                },
                DataSetConfig::Squad => {
                    let data:Vec<(u32,u32)> = self.label.clone().into_iter().map(|s|s.get_squad().unwrap()).collect();
                    let sp:Vec<u32> = data.clone().into_iter().map(|s| s.0).collect();
                    let ep:Vec<u32> = data.into_iter().map(|s| s.0).collect();
                    state.serialize_field("sp", &sp)?;
                    state.serialize_field("ep", &ep)?;

                },
                DataSetConfig::Mask { mask_length:_, mask:_ } => {
                    let data:Vec<Vec<i32>> = self.label.clone().into_iter().map(|s|s.get_vec_i32().unwrap()).collect();
                    state.serialize_field("labels", &data)?;
                },
                _ => todo!()
            }
            //state.serialize_field("label", &self.label)?;
            state.end()
    }
}