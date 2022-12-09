
use serde::{Deserialize, Serialize};



#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum DataSetConfig {
    Mask{mask_length:usize, mask:u32},
    Gpt,
    T5{number_spans: usize, mask_probability: f64 },
    MultiLabel{number_labels:usize},
    Squad,
    SingleClass,
    Python{mask_length:usize, context_shape:Vec<usize>},

    Bert
}


impl DataSetConfig {
    /*pub fn create_dataset(&self, batch_config:BatchConfig) -> DataSet {
        match self {
            //DataSetConfig::Mask { mask_length, mask } => {
            //    let masking_config = MaskingConfig{mask_length:mask_length.to_owned()};
            //    DataSet::Mask(MaskedData::new(masking_config, batch_config, mask.to_owned()))
            //}
            //DataSetConfig::Gpt => {
            //    DataSet::Gpt2(GptData::new(batch_config))
            //},
            //DataSetConfig::T5 { number_spans, mask_probability } => {
            //    let t5_config = T5Config{ number_spans: number_spans.to_owned(), 
            //        mask_probability: mask_probability.to_owned() };
            //    DataSet::T5(T5Data::new(t5_config, batch_config, vec![0;100]))
            //},
            
            DataSetConfig::Squad => {
                DataSet::Squad(SquadData::new(batch_config))
            },
            
            //DataSetConfig::Python {mask_length, context_shape} => {
            //    let python_config = PythonConfig{ mask_length:mask_length.to_owned(), context_shape:context_shape.to_owned() };
            //    DataSet::Python(PythonData::new(python_config, batch_config, 5))
            //},
            _ => todo!()
        }
    }*/



}