

use std::{sync::Arc};
use arrow::{array::{StringArray, StructArray, Int32Array, ListArray}, datatypes::Schema};

use crate::provider::arrow_transfer::ArrowGenerator;

use super::squad_data::SquadGeneral;

pub struct SquadArrowGenerator {
    
    pub q:usize,
    pub c:usize,
    pub a:usize,
}



impl ArrowGenerator for SquadArrowGenerator {
    type T = SquadGeneral;
    fn get_data(&self, data:&arrow::record_batch::RecordBatch) -> SquadGeneral {
        let question = StringArray::from(data.slice(0,1).column(self.q).data().to_owned()).value(0).to_string();
        let context = StringArray::from(data.slice(0,1).column(self.c).data().to_owned()).value(0).to_string();
        
        let answers = StructArray::from(data.slice(0,1).column(self.a).data().to_owned());
        let answer_list = ListArray::from(answers.column(0).data().to_owned()).value(0);
        let answer = StringArray::from(answer_list.data().to_owned()).value(0).to_string();

        // TODO : The start and end pointers don't properly work. I believe it's due to the character 
        let sp_list = ListArray::from(answers.column(1).data().to_owned()).value(0);
        let sp = Int32Array::from(sp_list.data().to_owned()).value(0);
        let ep = sp + answer.len() as i32;

         
        let char_vec: Vec<char> = context.chars().collect();
        let byte_vec = context.as_bytes();
        let mut offset:Option<usize> = None;
        if char_vec.len() != byte_vec.len() {
            offset = Some(byte_vec.len() - char_vec.len());            
        }
        
        

        let squad_data = SquadGeneral{ question: question, context: context, sp: sp as u32, ep: ep as u32, answer:Some(answer), offset:offset };
        return squad_data;
    }
}

impl SquadArrowGenerator {

    pub fn new(schema:&Arc<Schema>) -> Self {
        Self {
            q: schema.column_with_name("question").unwrap().0,
            c: schema.column_with_name("context").unwrap().0,
            a: schema.column_with_name("answers").unwrap().0,
        }
    }

    
}