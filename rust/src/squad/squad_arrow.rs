
use crate::provider::ProviderChannel;

use std::{fs::File};
use arrow::{ipc::reader::{StreamReader}, array::{StringArray, StructArray, Int32Array, ListArray}};
use tokio::sync::mpsc::Sender;

use super::squad_data::SquadGeneral;

pub struct SquadArrowLoader {
    pub q:usize,
    pub c:usize,
    pub a:usize,
    pub stream:StreamReader<File>
}

impl SquadArrowLoader {

    pub fn new(location:String) -> Self {
        println!("Location {}", location);
        let f = File::open(location);
        let stream_reader = StreamReader::try_new(f.unwrap(), None).unwrap();
        let schema = stream_reader.schema(); 

        Self {
            q: schema.column_with_name("question").unwrap().0,
            c: schema.column_with_name("context").unwrap().0,
            a: schema.column_with_name("answers").unwrap().0,
            stream:stream_reader
        }
    }

    pub async fn load_data(self, iterations:u64, tx:Sender<ProviderChannel<SquadGeneral>>) {

        let mut count = 0;
        for batch_wrap in self.stream {
            let batch = batch_wrap.unwrap();
            for x in 0..batch.num_rows() {
                let data = batch.slice(x, 1);
                let question = StringArray::from(data.slice(0,1).column(self.q).data().to_owned()).value(0).to_string();
                let context = StringArray::from(data.slice(0,1).column(self.c).data().to_owned()).value(0).to_string();
                let answers = StructArray::from(data.slice(0,1).column(self.a).data().to_owned());
                
                //let spa1 = StringArray::from(answers.column(0).data().to_owned()).value(0).to_owned();
                let answer_list = ListArray::from(answers.column(0).data().to_owned()).value(0);
                let answer = StringArray::from(answer_list.data().to_owned()).value(0).to_string();

                // TODO : The start and end pointers don't properly work. I believe it's due to the character 
                let sp_list = ListArray::from(answers.column(1).data().to_owned()).value(0);
                let sp = Int32Array::from(sp_list.data().to_owned()).value(0);
                let ep = sp + answer.len() as i32 + 1;

                
                //println!("AnswerDD {sp} {ep} {} {answer}", &context[sp as usize..ep as usize-1]);


                let squad_data = SquadGeneral{ question: question, context: context, sp: sp as u32, ep: ep as u32, answer:Some(answer) };
                //println!("Squad Generatl {:?}", squad_data);
                let _ = tx.send(ProviderChannel::Data(squad_data)).await;
                count += 1;
                if count == iterations {
                    let _ = tx.send(ProviderChannel::Complete).await;
                }
                
            }
        }

    
    }
}