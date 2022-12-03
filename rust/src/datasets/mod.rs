use serde::{Deserialize, Serialize};

use crate::tasks::{masking::{masked_data::MaskedData, gpt_data::GptData, t5_data::T5Data}, multi_label::multi_data::MultiData, squad::squad_data::SquadData, single_class::single_data::SingleClassData};
pub mod data_generator;


#[derive(Clone, Deserialize, Debug)]
pub enum DataSet {
    Mask(MaskedData),
    Gpt2(GptData),
    T5(T5Data),
    Multi(MultiData),
    Squad(SquadData),
    Single(SingleClassData)
}


impl DataSet {
    pub fn create_data(&mut self) -> DataSet {
        match self {
            DataSet::Mask(x) => DataSet::Mask(x.new_data()),
            DataSet::Gpt2(x) => DataSet::Gpt2(x.new_data()),
            DataSet::T5(x) => DataSet::T5(x.new_data()),
            DataSet::Multi(x) => DataSet::Multi(x.new_data()),
            DataSet::Squad(x) => DataSet::Squad(x.new_data()),
            DataSet::Single(x) => DataSet::Single(x.new_data()),
        }
    }

    pub fn put_data(&mut self, ids:&[u32]) -> bool {
        match self {
            DataSet::Mask(x) => x.put_data(ids),
            DataSet::Gpt2(x) => x.put_data(ids),
            DataSet::T5(x) => x.put_data(ids),
            DataSet::Multi(_x) => {true}//x.put_data(ids),
            DataSet::Squad(_x) => {true}//x.put_data(ids),
            DataSet::Single(_x) => {true}//x.put_data(ids),
        }
    }

    pub fn done(&self) -> bool {
        match self {
            DataSet::Mask(x) => x.done(),
            DataSet::Gpt2(x) => x.done(),
            DataSet::T5(x) => x.done(),
            DataSet::Multi(x) => x.done(),
            DataSet::Squad(x) => x.done(),
            DataSet::Single(x) => x.done(),
        }
    }

    pub fn remaining(&self) -> Option<Vec<u32>> {
        match self {
            DataSet::T5(x) => x.remaining.to_owned(),
            _ => None
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
                DataSet::T5(x) => x.serialize(serializer),
                DataSet::Multi(x) => x.serialize(serializer),
                DataSet::Squad(x) => x.serialize(serializer),
                DataSet::Single(x) => x.serialize(serializer),
            }
    }
}