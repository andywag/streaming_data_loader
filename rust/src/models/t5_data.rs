
use serde::{Serialize, Deserialize, ser::SerializeStruct};
use crate::{batcher::BatchConfig, datasets::dataset_config::{DataSetConfig}, models::simple_label::Label, tokenizer::{tokenizer_wrapper::TokenizerInfo, tokenizer_data::TokenizedData}};

use rand::prelude::*;
use rand_distr::{StandardNormal, Uniform};


#[derive(Debug, Clone, Deserialize)]
pub struct T5Data {
    pub input_ids:Vec<Vec<u32>>,
    pub attention_mask:Vec<Vec<u32>>,
    pub head_mask:Vec<Vec<Vec<u32>>>,
    pub decode_head_mask:Vec<Vec<Vec<u32>>>,
    pub labels:Vec<Vec<i32>>,
    index:usize,
    pub remaining:Option<Vec<u32>>,

    batch_config:BatchConfig, 
    dataset_config:DataSetConfig,
    tokenizer_info:TokenizerInfo,

}

impl T5Data {
    pub fn new(batch_config:BatchConfig, dataset_config:DataSetConfig, tokenizer_info:TokenizerInfo) -> Self{
        
        let (head_mask, decode_head_mask) = if let DataSetConfig::SpanHier { avg_span_prob, context_size } = dataset_config {
            let max_label_length = (2.1*avg_span_prob*batch_config.sequence_length as f64).round() as usize;
            let (c,b,s) = (context_size, batch_config.batch_size, batch_config.sequence_length);
            let head_mask = vec![vec![vec![255;s];c];b];
            let decode_head_mask = vec![vec![vec![255;max_label_length];c];b];
            (head_mask, decode_head_mask)
        }
        else {
            (vec![vec![vec![]]], vec![vec![vec![]]])
        };

        Self {
            input_ids: vec![vec![0;batch_config.sequence_length];batch_config.batch_size],
            attention_mask: vec![vec![1;batch_config.sequence_length];batch_config.batch_size],
            head_mask,
            decode_head_mask,
            labels:vec![vec![-100;batch_config.sequence_length/4]; batch_config.batch_size],
            
            index:0, 
            remaining:None,

            batch_config, 
            dataset_config,
            tokenizer_info,

        }
    }




    pub fn write_input(&mut self, data:&TokenizedData, extra:Option<u32>, context_size:usize, ip:usize, op:usize) -> (usize,usize) {
        if ip >= data.ids.len() || op >= self.batch_config.sequence_length {
            return (ip,op)
        }
        self.input_ids[self.index][op] = extra.unwrap_or(data.ids[ip]);  
        for y in 0..context_size {
            self.head_mask[self.index][y][op] = data.attention_mask[y][ip];
        }
        (extra.map(|_| ip).unwrap_or(ip+1), op+1)
    }

    pub fn write_label(&mut self, data:&TokenizedData, extra:Option<u32>, context_size:usize, ip:usize, lop:usize) -> (usize, usize) {
        if ip >= data.ids.len() || lop >= self.labels[0].len() {
            return (ip,lop)
        }
        self.labels[self.index][lop] = extra.unwrap_or(data.ids[ip]) as i32;  
        for y in 0..context_size {
            self.decode_head_mask[self.index][context_size-y-1][lop] = data.attention_mask[y][ip];
        }
        (extra.map(|_| ip).unwrap_or(ip+1), lop+1)
        
    }




    pub fn put_tokenized_data(&mut self, data:TokenizedData, _label:Option<Label>) -> bool{
        // TODO : Fix this condition by limiting data size
        //log::info!("Dataset {:?}", data.ids);

        if data.ids.len() < 64 {
            return false;
        }

        let data_config_clone = self.dataset_config.clone();
        if let DataSetConfig::SpanHier { avg_span_prob, context_size} = data_config_clone {
            let mut op = 0; // Index into output data
            let mut ip = 0; // Index into input data
            let mut lip = 0; // Label Pointer
            let mut lop = 0;

            let uniform = Uniform::new(0.0,1.0);
            for (_i,gap) in data.gaps.clone().into_iter().enumerate() {
                // Count out the number of masked tokens 
                // Normal Distribution Approximation Failed due to small samples (or my poor coding)
                let mut span_length = 0;
                for _ in 0..gap {
                    let rv = thread_rng().sample(uniform);
                    if rv < avg_span_prob {
                        span_length += 1;
                    }
                }
                
                // Create the starting point of the label or leave it at the end of the gap
                let sp = if span_length > 0 {
                    let rv = thread_rng().sample(uniform);
                    (rv*(gap-span_length) as f64).round() as usize
                }
                else {
                    gap
                };
                
                if op + gap + 8 >= self.batch_config.sequence_length || ip + gap >= self.batch_config.sequence_length {
                    break;
                }
                for x in 0..gap {
                    
                    if x < sp {
                        (ip,op) = self.write_input(&data, None, context_size, ip, op);
                    }
                    else if x == sp {
                        let extra = self.tokenizer_info.extra[lip]; lip += 1;
                        (ip,op) = self.write_input(&data, Some(extra), context_size, ip, op);
                        (ip,lop) = self.write_label(&data, Some(extra), context_size, ip, lop);
                        (ip,lop) = self.write_label(&data, None, context_size, ip, lop);
                    }
                    else if x > sp && x < sp + span_length {
                        (ip,lop) = self.write_label(&data, None, context_size, ip, lop);
                    }
                    else {
                        (ip,op) = self.write_input(&data, None, context_size, ip, op);
                    }
                }
                //log::info!("Done {} {} {} {} {}", gap, ip, op, lip, lop);
            }
            self.write_label(&data, Some(self.tokenizer_info.extra[lip]), context_size, ip, lop);
            //log::info!("Remaining {}", op);
            for x in op..self.batch_config.sequence_length {
                self.attention_mask[self.index][x] = 0;
            }
            //log::info!("Here {} {} {} {}", ip, op, lip, lop);
            //log::info!("Done {} {} {} {}", ip, op, lip, lop);
        }
        else {
           log::error!("Requires Span Configuration for this Mode");
        }
        
        //log::info!("IdsOut {:?}", self.input_ids);
        //log::info!("Label {:?}", self.labels);
        self.index += 1;
        self.done()
    }

    pub fn put_data(&mut self, ids:Vec<u32>, _label:Option<Label>) -> bool{

        // TODO : Add Round and Proper Std scaling
        pub fn random_data_gap(avg_span_gap:f64) -> usize {
            let val: f64 = thread_rng().sample(StandardNormal);
            let distance = avg_span_gap - val;
            distance as usize
        }

        // TODO : Add Round and Proper Std Scaling
        pub fn random_data_size(avg_span_size:f64) -> usize {
            let val: f64 = thread_rng().sample(StandardNormal);
            let distance = avg_span_size - val;
            std::cmp::max(distance as usize,1)
        }

        let (avg_span_gap, avg_span_size) = match self.dataset_config {
            DataSetConfig::Span { avg_span_gap, avg_span_size } => (avg_span_gap, avg_span_size),
            _ => panic!("Task Requires Different Configuration")
        };


        let mut ip:usize = 0;
        while self.index < self.batch_config.batch_size {
            let mut lp = 0;
            let mut ap = 0;
            let mut pass = 0;
            while lp < self.batch_config.sequence_length {
                // Create the gap between spans
                let mut data_gap =random_data_gap(avg_span_gap);
                data_gap = std::cmp::min(data_gap, self.batch_config.sequence_length - lp);
                data_gap = std::cmp::min(data_gap, ids.len() - ip);
                if data_gap > 0 {
                    self.input_ids[self.index][lp..lp+data_gap].clone_from_slice(&ids[ip..ip+data_gap]);
                    lp += data_gap; ip += data_gap;
                }
                // Create the Span of Masked Values
                data_gap = random_data_size(avg_span_size);
                data_gap = std::cmp::min(data_gap, self.batch_config.sequence_length - lp);
                data_gap = std::cmp::min(data_gap, ids.len() - ip);

                if data_gap > 0 {
                    self.input_ids[self.index][lp] = self.tokenizer_info.extra[pass]; // TODO : Mask Token
                    self.labels[self.index][ap] = self.tokenizer_info.extra[pass] as i32; // TODO : Mask Token
                    for i in 0..data_gap {
                        self.labels[self.index][ap+i+1] = ids[ip + i] as i32;
                    }
                    lp += 1; ip += data_gap; ap += data_gap + 1;
                }
                if ids.len() <= ip {
                    for x in 0..ip-ids.len() {
                        self.attention_mask[self.index][lp + x] = 0;
                    }
                    
                    self.labels[self.index][ap] = self.tokenizer_info.extra[pass + 1] as i32;
                    self.index += 1;
                    return false;
                }
                pass += 1;
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
            log::info!("Sending Data");

            let mut state = serializer.serialize_struct("T5Data", 3)?;
            state.serialize_field("input_ids", &self.input_ids)?;
            state.serialize_field("attention_mask", &self.attention_mask)?;
            state.serialize_field("labels", &self.labels)?;
            if let DataSetConfig::SpanHier { avg_span_prob:_, context_size:_} = self.dataset_config {
                state.serialize_field("head_mask", &self.head_mask)?;
                state.serialize_field("decoder_head_mask", &self.decode_head_mask)?;
            }
            state.end()
    }
}