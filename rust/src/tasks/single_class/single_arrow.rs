

use std::{sync::Arc};
use arrow::{array::{StringArray, Int32Array}, datatypes::Schema};

use crate::provider::arrow_transfer::ArrowGenerator;

use super::single_data::SingleClassTransport;



pub struct SingleClassArrowGenerator {
    pub t:usize, // Text Location
    pub l:usize, // Label Location
}

impl ArrowGenerator for SingleClassArrowGenerator {
    type T = SingleClassTransport;
    fn get_data(&self, data:&arrow::record_batch::RecordBatch) -> Self::T {
        let text = StringArray::from(data.slice(0,1).column(self.t).data().to_owned()).value(0).to_string();
        let label = Int32Array::from(data.slice(0,1).column(self.l).data().to_owned()).value(0);

        //log::info!("Here {} {}", text, label);
        let data = Self::T{text:text, label:label as u32};
        return data;
    }
}

impl SingleClassArrowGenerator {

    pub fn new(schema:&Arc<Schema>) -> Self {
        Self {
            t: schema.column_with_name("text").unwrap().0,
            l: schema.column_with_name("label").unwrap().0,
        }
    }

    
}