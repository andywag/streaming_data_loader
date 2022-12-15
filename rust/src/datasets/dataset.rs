
use serde::{Deserialize, Serialize};

use crate::{tasks::{squad::squad_data::SquadData}, models::{simple_label::Label, bert_data::BertData, gpt_data::GptData, t5_data::T5Data, hier_bert_data::BertHierData}};

#[derive(Clone, Deserialize, Debug)]
pub enum DataSet {
    Gpt2(GptData),
    T5(T5Data),
    Multi,
    Squad(SquadData),
    Single,

    Bert(BertData),
    BertHier(BertHierData)
}

impl From<BertData> for DataSet {
    fn from(x: BertData) -> Self {DataSet::Bert(x)}
}
impl From<GptData> for DataSet {
    fn from(x: GptData) -> Self {DataSet::Gpt2(x)}
}
impl From<T5Data> for DataSet {
    fn from(x: T5Data) -> Self {DataSet::T5(x)}
}
impl From<BertHierData> for DataSet {
    fn from(x: BertHierData) -> Self {DataSet::BertHier(x)}
}



impl DataSet {
    /*pub fn create_data(&mut self) -> DataSet {
        match self {
            DataSet::Mask(x) => DataSet::Mask(x.new_data()),
            //DataSet::Gpt2(x) => DataSet::Gpt2(x.new_data()),
            //DataSet::T5(x) => DataSet::T5(x.new_data()),
            DataSet::Multi => todo!(),
            DataSet::Squad(x) => DataSet::Squad(x.new_data()),
            DataSet::Single => todo!(),
            //DataSet::Python(x) => DataSet::Python(x.new_data()),
            _ => todo!()
        }
    }*/

    pub fn put_full_data(&mut self, data:Vec<u32>, _alt_data:Option<Vec<u32>>, label:Option<Label>) -> bool {
        match self {
            DataSet::Bert(x) => {
                x.put_data(data, label)
            },
            DataSet::Gpt2(x) => {
                x.put_data(data, label)
            },
            DataSet::T5(x) => {
                x.put_data(data, label)
            }
            _ => {
                false
            }
        }
    }
    
    pub fn put_data(&mut self, _ids:&[u32]) -> bool {
        match self {
            //DataSet::Gpt2(x) => x.put_data(ids),
            //DataSet::T5(x) => x.put_data(ids),
            DataSet::Squad(_x) => {true}//x.put_data(ids),
            DataSet::BertHier(_) => todo!(),
            _ => todo!()
        }
    }

    pub fn done(&self) -> bool {
        match self {
            DataSet::Gpt2(x) => x.done(),
            DataSet::T5(x) => x.done(),
            DataSet::Squad(x) => x.done(),
            DataSet::BertHier(x) => x.done(),
            DataSet::Bert(x) => x.done(),

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
                DataSet::Gpt2(x) => x.serialize(serializer),
                DataSet::T5(x) => x.serialize(serializer),
                DataSet::Squad(x) => x.serialize(serializer),
                DataSet::BertHier(x) => x.serialize(serializer),
                DataSet::Bert(x) => x.serialize(serializer),
            _ => todo!()
            }
    }
}