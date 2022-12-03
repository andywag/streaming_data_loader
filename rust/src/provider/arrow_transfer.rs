use arrow::{ipc::{reader::StreamReader}, datatypes::Schema};
use rand::{seq::SliceRandom, thread_rng};
use tokio::sync::mpsc::Sender;
use std::{fs::File, sync::Arc};

use crate::tasks::DatasetInfo;

use super::{ProviderChannel, provider_config::ProviderConfig};


// Trait to support generic Loading of an Arror File into data Type 

pub trait ArrowGenerator {
    type T;
    fn get_data(&self, batch:&arrow::record_batch::RecordBatch) -> Self::T;
}

// Top Level Structure To Handle Loading the Arrow File
pub struct ArrowTransfer<T:Clone> {
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

impl <T:Clone>ArrowTransfer<T> {
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

    pub async fn load_flat_data(&mut self, capacity:usize, tx:&Sender<ProviderChannel<T>>, iterations:Option<usize>, epochs:Option<usize>, shuffle:bool) {
        let stream = create_reader(self.location.clone()); //mem::take(&mut self.stream).unwrap();
        
        let mut data_storage = Vec::<T>::with_capacity(capacity);

        // Download and Store the Full Set of Data from the Arrow File
        for batch_wrap in stream {
            let batch = batch_wrap.unwrap();
            for x in 0..batch.num_rows() {
                let data = batch.slice(x, 1);
                let result_data = self.generator.as_ref().unwrap().get_data(&data);
                data_storage.push(result_data);
            }
        }
        let mut iteration_count = 0;
        let mut epoch_count = 0;
        let mut positions:Vec<usize> = (0..capacity).collect();

        loop {
            if shuffle {
                positions.shuffle(&mut thread_rng());
            }
            for x in 0..capacity {
                let data = data_storage[positions[x]].clone();
                let _ = tx.send(ProviderChannel::Data(data)).await;
                iteration_count += 1;
                if iterations.is_some() && iteration_count == iterations.unwrap() {                    
                    return;
                }
            }
            epoch_count += 1;
            if epochs.is_some() && epoch_count == epochs.unwrap() {
                return;
            }
        }
    }

    pub async fn load_stream_data(&mut self, tx:&Sender<ProviderChannel<T>>, iterations:Option<usize>, epochs:Option<usize>, shuffle:bool) {
        let mut iteration_count = 0;
        let mut epoch_count = 0;

        loop {
            let stream = create_reader(self.location.clone()); 
        

            for batch_wrap in stream {
                let batch = batch_wrap.unwrap();
                let mut positions:Vec<usize> = (0..batch.num_rows()).collect();
                if shuffle {
                    positions.shuffle(&mut thread_rng());
                }
                for x in 0..batch.num_rows() {
                    let data = batch.slice(positions[x], 1);
                   
                    let result_data = self.generator.as_ref().unwrap().get_data(&data);
                    let _ = tx.send(ProviderChannel::Data(result_data)).await;
                    iteration_count += 1;
                    if iterations.is_some() && iterations.unwrap() == iteration_count {
                        return;
                    }
                }
            }
            epoch_count += 1;
            
            if epochs.is_some() && epochs.unwrap() == epoch_count {
                return;
            }
        }
        
    }

    // Async Load of the Data. Runs through all of the data contained in the arrow file
    // TODO : Support Epochs and longer simulation times
    pub async fn load_data(&mut self, config:ProviderConfig, tx:Sender<ProviderChannel<T>>) {

        let mut it:Option<usize> = None;
        let mut ep:Option<usize> = None;
        match config.length {
            crate::provider::provider_config::ProviderLength::Iterations { iterations } => {it = Some(iterations)},
            crate::provider::provider_config::ProviderLength::Epochs { epochs } => {ep = Some(epochs)},
        }
        let shuffle = match config.shuffle {
            Some(x) => x,
            None => true
        };
        let flatten = match config.flatten {
            Some(x) => x,
            None => true
        };

        // Send the Information about the dataset through the channel
        let _ = tx.send(ProviderChannel::Info(DatasetInfo{ name: "dataset".to_string(), length: self.num_rows })).await;

        if flatten {
            self.load_flat_data(self.num_rows as usize, &tx, it, ep, shuffle).await;
        }
        else {
            self.load_stream_data(&tx, it, ep, shuffle).await;
        }
        let _ = tx.send(ProviderChannel::Complete).await;
        
        // Send the dataset information over the channel
        log::info!("Finished Data Loader");

        
    }

}

// Convenience Function to Create Provider



