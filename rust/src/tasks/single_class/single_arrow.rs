

use std::{sync::Arc};
use arrow::{array::{StringArray,Int64Array}, datatypes::{Schema}};

use crate::{provider::arrow_transfer::ArrowGenerator, models::simple_transport::SimpleTransport};




pub struct SingleClassArrowGenerator {
    pub t:usize, // Text Location
    pub l:usize, // Label Location
}

impl ArrowGenerator for SingleClassArrowGenerator {
    type T = SimpleTransport;
    fn get_data(&self, data:&arrow::record_batch::RecordBatch) -> Self::T {
        let text = StringArray::from(data.slice(0,1).column(self.t).data().to_owned()).value(0).to_string();
        let label3 = data.slice(0,1).column(self.l).data().to_owned();
        let label2 = Int64Array::from(label3).value(0) as u32;
        
        //let data = Self::T{text:text, label:label2 as u32};
        let data = Self::T{data:(text,None).into(), label:Some(label2.into())};

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