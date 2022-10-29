use tokio::sync::mpsc::Receiver;

use crate::{provider::ProviderChannel, transport::ZmqChannel};

pub trait Batcher {
    type S;
    type T;
    fn create_sync_batch(&mut self, data:Self::S) -> Option<Self::T>;
}

pub async fn create_batch<S,T>(mut rx:Receiver<ProviderChannel<S>>, 
    tx_transport:tokio::sync::mpsc::Sender<ZmqChannel<T>>,
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
            ProviderChannel::Complete => {
                let _ = tx_transport.send(ZmqChannel::Complete).await;
                println!("Finished Tokenizer");
                return;
            },
            ProviderChannel::Data(x) => {
                let batch = batcher.create_sync_batch(x);
                if batch.is_some() {
                    println!("Sending Batch");
                    let real_batch = batch.unwrap();
                    let _result = tx_transport.send(ZmqChannel::Data(real_batch)).await;
                }
            },
        }

    }
    
}




