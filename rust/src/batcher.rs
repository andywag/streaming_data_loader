use std::sync::Arc;

use serde::{Serialize, Deserialize};
use tokio::{sync::mpsc::Receiver, task::{JoinHandle, self}};

use crate::{provider::ProviderChannel};
use pyo3::prelude::*;


#[derive(Deserialize, Serialize, Debug, Clone, FromPyObject)]

pub struct BatchConfig {
    pub batch_size:usize,
    pub sequence_length:usize
}

impl BatchConfig {
    pub fn create_vector<T:Clone>(&self, value:T) -> Vec<Vec<T>> {
        vec![vec![value;self.sequence_length];self.batch_size]
    }
    pub fn create_vector_1d<T:Clone>(&self, value:T) -> Vec<T> {
        vec![value;self.batch_size]
    }
}


pub trait Batcher {
    type S;
    type T;
    fn create_sync_batch(&mut self, data:Self::S) -> Option<Self::T>;
    fn get_working_batch(&mut self) -> Option<Self::T>;
}

pub async fn create_batch<S,T>(mut rx:Receiver<ProviderChannel<S>>, 
    tx_transport:tokio::sync::mpsc::Sender<ProviderChannel<T>>,
    mut batcher:Box<dyn Batcher<S = S, T = T> + Send>
    ) {

    
    loop {
        // Wait for Data from the Transmit Link
        let data_option = rx.recv().await;
        // Channel is shutdown if the receive data is None           
        if data_option.is_none() {
           break
        }
        // Match the input to check if the stream is complete and send the complete command forward
        match data_option.unwrap() {
            ProviderChannel::Info(x) => {
                log::info!("Sending Dataset Info");
                let _ = tx_transport.send(ProviderChannel::Info(x)).await;
            }
            ProviderChannel::Complete => {
                // Flush the Current Packet
                let current = batcher.get_working_batch();
                match current {
                    Some(x) => {
                        let _ = tx_transport.send(ProviderChannel::Data(x)).await;
                    },
                    None => {}
                }
                let _ = tx_transport.send(ProviderChannel::Complete).await;
                
                break;
            },
            ProviderChannel::Data(x) => {
                let batch = batcher.create_sync_batch(x);
                if batch.is_some() {
                    // Batch");
                    log::info!("Sending Data");
                    let real_batch = batch.unwrap();
                    let _result = tx_transport.send(ProviderChannel::Data(real_batch)).await;
                }
            },
        }
    }
    log::info!("Finished Batcher");
}

pub async fn create_batcher<P:Send + 'static, D:Serialize+Send+'static>(value:Arc<serde_yaml::Value>,
    generator:Box<dyn Fn(&Arc<serde_yaml::Value>)-> Box<dyn Batcher<S=P,T=D> + Send>>,
    rx:tokio::sync::mpsc::Receiver<ProviderChannel<P>>, 
    tx:tokio::sync::mpsc::Sender<ProviderChannel<D>>) -> JoinHandle<()> {
    // Create the Data Provider
    let generator = generator(&value);

    let join_tokenizer = task::spawn(async move {
        let result = create_batch(rx, tx, generator);
        result.await;
    });
    join_tokenizer
}


