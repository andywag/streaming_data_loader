use pyo3::{pyclass, Python, types::PyDict};

use crate::{models::{t5_data::T5Data, bert_data::BertData, simple_label::Label}, datasets::{dataset::DataSet, dataset_config::DataSetConfig}};



#[pyclass]
pub struct PyT5Data {
    pub input_ids:Vec<Vec<u32>>,
    pub attention_mask:Vec<Vec<u32>>,
    pub head_mask:Vec<Vec<Vec<u32>>>,
    pub decode_head_mask:Vec<Vec<Vec<u32>>>,
    pub labels:Vec<Vec<i32>>,
}

impl PyT5Data {
    pub fn from(data:T5Data) -> PyT5Data {
        Self {
            input_ids: data.input_ids,
            attention_mask: data.attention_mask,
            head_mask: data.head_mask,
            decode_head_mask: data.decode_head_mask,
            labels: data.labels,
        }
    }
}

#[pyclass]
#[pyo3(frozen)]
pub struct PyBertData {
    #[pyo3(get)]
    pub input_ids:Vec<Vec<u32>>,
    #[pyo3(get)]
    pub attention_mask:Vec<Vec<u32>>,
    #[pyo3(get)]
    pub token_type_ids:Vec<Vec<u32>>,
    //pub label:Vec<Label>,
}

impl PyBertData {
    pub fn from(data:BertData) -> PyBertData {
        Self {
            input_ids: data.input_ids,
            attention_mask: data.attention_mask,
            token_type_ids: data.token_type_ids
        }
    }
}



fn convert_labels(dict:&PyDict, label:Vec<Label>, dataset_config:DataSetConfig) {
    let _result = match dataset_config {
        DataSetConfig::SingleClass => {
            let data:Vec<u32> = label.clone().into_iter().map(|s|s.get_single().unwrap()).collect();
            dict.set_item("label", &data)
        },
        DataSetConfig::MultiLabel { number_labels: _  } => {
            let data:Vec<Vec<f32>> = label.clone().into_iter().map(|s|s.get_multi_f32().unwrap()).collect();
            dict.set_item("labels", &data)
        },
        DataSetConfig::Squad => {
            let data:Vec<(u32,u32)> = label.clone().into_iter().map(|s|s.get_squad().unwrap()).collect();
            let sp:Vec<u32> = data.clone().into_iter().map(|s| s.0).collect();
            let ep:Vec<u32> = data.into_iter().map(|s| s.0).collect();
            let _ = dict.set_item("sp", &sp);
            dict.set_item("ep", &ep)

        },
        DataSetConfig::Mask { mask_length:_, mask:_ } => {
            let data:Vec<Vec<i32>> = label.clone().into_iter().map(|s|s.get_vec_i32().unwrap()).collect();
            dict.set_item("labels", &data)
        },
        _ => todo!()
    };
}

pub fn convert_data_set(data:DataSet, py:Python, dataset_config:DataSetConfig) -> &PyDict {
    match data {
        DataSet::Gpt2(_) => todo!(),
        DataSet::T5(_x) => todo!(),
        DataSet::Multi => todo!(),
        DataSet::Squad(_) => todo!(),
        DataSet::Single => todo!(),
        DataSet::Bert(x) => {
            let dict = PyDict::new(py);
            let _ = dict.set_item("input_ids", x.input_ids);
            let _ = dict.set_item("attention_mask", x.attention_mask);
            let _ = dict.set_item("token_type_ids", x.token_type_ids);
            convert_labels(dict, x.label, dataset_config);
            return dict
        }
        DataSet::BertHier(_) => todo!(),
    }
}