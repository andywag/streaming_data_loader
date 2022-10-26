use arrow::{ipc::{reader::StreamReader}, datatypes::Schema};
use tokio::sync::mpsc::Sender;
use std::{fs::File, sync::Arc, mem};

use super::ProviderChannel;


// Trait to support generic Loading of an Arror File into data Type 

pub trait ArrowGenerator {
    type T;
    fn get_data(&self, batch:&arrow::record_batch::RecordBatch) -> Self::T;
}

// Top Level Structure To Handle Loading the Arrow File
pub struct ArrowTransfer<T> {
    pub stream:Option<StreamReader<File>>,
    pub schema:Arc<Schema>,
    pub generator:Option<Box<dyn ArrowGenerator<T=T> + Send>>
}

impl <T>ArrowTransfer<T> {
    // Load the Arrow File and parse the schema block
    pub fn new(location:String) -> Self {
        println!("Loading Arrow File {}", location);
        let f = File::open(location);
        let stream_reader = StreamReader::try_new(f.unwrap(), None).unwrap();
        let schema = stream_reader.schema(); 

        Self {
            stream:Some(stream_reader),
            schema:schema,
            generator:None
        }
    }

    // Async Load of the Data. Runs through all of the data contained in the arrow file
    // TODO : Support Epochs and longer simulation times
    pub async fn load_data(&mut self, iterations:u64, tx:Sender<ProviderChannel<T>>) {

        let mut count = 0;
        let stream = mem::take(&mut self.stream).unwrap();
        
        for batch_wrap in stream {
            let batch = batch_wrap.unwrap();
            for x in 0..batch.num_rows() {
                let data = batch.slice(x, 1);
               
                let result_data = self.generator.as_ref().unwrap().get_data(&data);
                let _ = tx.send(ProviderChannel::Data(result_data)).await;
                count += 1;
                if count == iterations {
                    let _ = tx.send(ProviderChannel::Complete).await;
                }
                
            }
        }   
    }

}

// Convenience Function to Create Provider



