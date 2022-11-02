use arrow::{ipc::{reader::StreamReader}, datatypes::Schema};
use tokio::sync::mpsc::Sender;
use std::{fs::File, sync::Arc};

use crate::datasets::DatasetInfo;

use super::{ProviderChannel, ProviderConfig};


// Trait to support generic Loading of an Arror File into data Type 

pub trait ArrowGenerator {
    type T;
    fn get_data(&self, batch:&arrow::record_batch::RecordBatch) -> Self::T;
}

// Top Level Structure To Handle Loading the Arrow File
pub struct ArrowTransfer<T> {
    location:String,
    pub schema:Arc<Schema>,
    pub generator:Option<Box<dyn ArrowGenerator<T=T> + Send>>,
    pub num_rows:u32
}

fn create_reader(location:String) -> StreamReader<File> {
    let f = File::open(location.clone());
    let stream_reader = StreamReader::try_new(f.unwrap(), None).unwrap();
    stream_reader
}

impl <T>ArrowTransfer<T> {
    // Load the Arrow File and parse the schema block
    pub fn new(location:String, length:u32) -> Self {
        println!("Loading Arrow File {}", location);
        
        let stream_reader = create_reader(location.clone());
        let schema = stream_reader.schema(); 
        
        Self {
            location:location,
            schema:schema,
            generator:None,
            num_rows:length
        }
    }

    // Async Load of the Data. Runs through all of the data contained in the arrow file
    // TODO : Support Epochs and longer simulation times
    pub async fn load_data(&mut self, config:ProviderConfig, tx:Sender<ProviderChannel<T>>) {

        let iterations = if let ProviderConfig::Iterations(x) = &config{Some(x.iterations)} else {None}; 
        let epochs = if let ProviderConfig::Epochs(x) = config{Some(x.epochs)} else {None}; 

        // Send the dataset information over the channel
        let _ = tx.send(ProviderChannel::Info(DatasetInfo{ name: "dataset".to_string(), length: self.num_rows })).await;

        let mut iteration_count = 0;
        let mut epoch_count = 0;
        let mut finished = false;

        loop {
            let stream = create_reader(self.location.clone()); //mem::take(&mut self.stream).unwrap();
        

            for batch_wrap in stream {
                let batch = batch_wrap.unwrap();
                for x in 0..batch.num_rows() {
                    let data = batch.slice(x, 1);
                   
                    let result_data = self.generator.as_ref().unwrap().get_data(&data);
                    let _ = tx.send(ProviderChannel::Data(result_data)).await;
                    iteration_count += 1;
                    if iterations.is_some() && iterations.unwrap() == iteration_count {
                        let _ = tx.send(ProviderChannel::Complete).await;
                        finished = true;
                        break;
                    }
                }
                if finished {break;}
            }
            if finished {break;}
            epoch_count += 1;
            
            if epochs.is_some() && epochs.unwrap() == epoch_count {
                let _ = tx.send(ProviderChannel::Complete).await;
                if finished {break;}
            }
        }
        

    }

}

// Convenience Function to Create Provider



