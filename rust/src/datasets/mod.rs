use serde::{Deserialize, Serialize};

use crate::tasks::{masking::{masked_data::MaskedData, gpt_data::GptData}};
pub mod data_generator;


#[derive(Clone, Deserialize)]
pub enum DataSet {
    Mask(MaskedData),
    Gpt2(GptData)
}


impl DataSet {
    pub fn create_data(&mut self) -> DataSet {
        match self {
            DataSet::Mask(x) => DataSet::Mask(x.new_data()),
            DataSet::Gpt2(x) => DataSet::Gpt2(x.new_data()),
            

        }
    }

    pub fn put_data(&mut self, ids:&[u32]) -> bool {
        match self {
            DataSet::Mask(x) => x.put_data(ids),
            DataSet::Gpt2(x) => x.put_data(ids),
        }
    }

    pub fn done(&self) -> bool {
        match self {
            DataSet::Mask(x) => x.done(),
            DataSet::Gpt2(x) => x.done(),
        }
    }
}

impl Serialize for DataSet {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
            match self {
                DataSet::Mask(x) => x.serialize(serializer),
                DataSet::Gpt2(x) => x.serialize(serializer),
            }
    }
}