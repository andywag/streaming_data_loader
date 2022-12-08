

use std::{sync::Arc};
use arrow::{array::{StringArray, ListArray, Int64Array}, datatypes::Schema};

use crate::{provider::arrow_transfer::ArrowGenerator, models::simple_transport::SimpleTransport};



pub struct MultiArrowGenerator {
    pub t:usize,
    pub l:usize,
}

impl ArrowGenerator for MultiArrowGenerator {
    type T = SimpleTransport;
    fn get_data(&self, data:&arrow::record_batch::RecordBatch) -> Self::T {
        let text = StringArray::from(data.slice(0,1).column(self.t).data().to_owned()).value(0).to_string();
        
        // Really Painful Code to Extra the labels from the arrow file
        // TODO : Need Generalized Methods to Extract Items from Arrow
        let labels3 = ListArray::from(data.column(self.l).slice(0,1).data().to_owned()).value(0);
        let labels2 = Int64Array::from(labels3.data().to_owned()); //values().into_iter().collect();
        let labels1:Vec<Option<i64>> = labels2.into_iter().collect();
        let labels:Vec<u32> = labels1.into_iter().map(|e| e.unwrap() as u32).collect();

        //let squad_data = Self::T{text:text, labels:labels};
        let data = SimpleTransport{ data: (text,None).into(), label: Some(labels.into()) };

        return data;
    }
}

impl MultiArrowGenerator {

    pub fn new(schema:&Arc<Schema>) -> Self {
        Self {
            t: schema.column_with_name("sentence").unwrap().0,
            l: schema.column_with_name("labels").unwrap().0,
        }
    }

    
}