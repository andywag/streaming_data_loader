
use crate::config::{TrainingConfig};

use super::masking::masking_cases::MaskingCases;
use super::masking::{masking_cases};
use super::multi_label::multi_cases;
use super::python::python_cases;
use super::single_class::single_cases;
use super::squad::squad_cases;



pub enum BasicCases {
    Bert, 
    Roberta,
    Gpt,
    T5,
    Squad,
    Multi,
    Single,
    Python,
    PythonContext,
    PythonSpan,
}

impl BasicCases {
    pub fn get_config(&self, test:bool) -> TrainingConfig {

        match self {
            BasicCases::Bert => masking_cases::get_case(MaskingCases::Bert, test),
            BasicCases::Roberta => masking_cases::get_case(MaskingCases::Bert, test),
            BasicCases::Gpt => masking_cases::get_case(MaskingCases::Gpt, test),
            BasicCases::T5 => masking_cases::get_case(MaskingCases::T5, test),
            BasicCases::Squad => squad_cases::get_case(test),
            BasicCases::Multi => multi_cases::get_case(test),
            BasicCases::Single => single_cases::get_case(single_cases::Cases::Imdb, test),
            BasicCases::Python => python_cases::get_case(python_cases::Cases::Basic, test),
            BasicCases::PythonContext => python_cases::get_case(python_cases::Cases::Context, test),
            BasicCases::PythonSpan => python_cases::get_case(python_cases::Cases::Span, test),
        }
    }
}