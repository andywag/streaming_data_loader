
use serde::{Deserialize, Serialize};

use crate::{tasks::{masking::{masked_data::MaskedData, gpt_data::GptData, t5_data::T5Data},  squad::squad_data::SquadData, python::python_data::PythonData}, models::{simple_label::Label, bert::data::BertData}};

#[derive(Clone, Deserialize, Debug)]
pub enum DataSet {
    Mask(MaskedData),
    Gpt2(GptData),
    T5(T5Data),
    Multi,
    Squad(SquadData),
    Single,
    Python(PythonData),

    Bert(BertData)
}

impl From<BertData> for DataSet {
    fn from(x: BertData) -> Self {DataSet::Bert(x)}
}


impl DataSet {
    pub fn create_data(&mut self) -> DataSet {
        match self {
            DataSet::Mask(x) => DataSet::Mask(x.new_data()),
            DataSet::Gpt2(x) => DataSet::Gpt2(x.new_data()),
            DataSet::T5(x) => DataSet::T5(x.new_data()),
            DataSet::Multi => todo!(),
            DataSet::Squad(x) => DataSet::Squad(x.new_data()),
            DataSet::Single => todo!(),
            DataSet::Python(x) => DataSet::Python(x.new_data()),
            _ => todo!()
        }
    }

    pub fn put_full_data(&mut self, data:Vec<u32>, _alt_data:Option<Vec<u32>>, label:Option<Label>) -> bool {
        match self {
            DataSet::Bert(x) => {
                x.put_data(data, label)
            }
            _ => {
                false
            }
        }
    }
    
    pub fn put_data(&mut self, ids:&[u32]) -> bool {
        match self {
            DataSet::Mask(x) => x.put_data(ids),
            DataSet::Gpt2(x) => x.put_data(ids),
            DataSet::T5(x) => x.put_data(ids),
            DataSet::Squad(_x) => {true}//x.put_data(ids),
            DataSet::Python(_) => {true}, //x.put_data(ids),
            _ => todo!()
        }
    }

    pub fn done(&self) -> bool {
        match self {
            DataSet::Mask(x) => x.done(),
            DataSet::Gpt2(x) => x.done(),
            DataSet::T5(x) => x.done(),
            DataSet::Squad(x) => x.done(),
            DataSet::Python(x) => x.done(),
            _ => todo!()
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
                DataSet::Squad(x) => x.serialize(serializer),
                DataSet::Python(x) => x.serialize(serializer),
                DataSet::Bert(x) => x.serialize(serializer),
            _ => todo!()
            }
    }
}