
use std::sync::Arc;

use serde::Serialize;
use tokio::{task::{JoinHandle, self}, sync::mpsc::Receiver};

use crate::{provider::ProviderChannel, config::TrainingConfig};


pub trait EndPoint<T> {
    fn receive(&mut self, _data:T) -> bool {
        return true;
    }
}

pub struct DefaultTestEndPoint {}
impl <T>EndPoint<T> for DefaultTestEndPoint {
    fn receive(&mut self, _data:T) -> bool {
        return true;
    }
}
// Create the Endpoint for Squad
pub fn default_endpoint<T>(_config:TrainingConfig) -> Box<dyn crate::transport::test_endpoint::EndPoint<T> + Send> {
    return Box::new(DefaultTestEndPoint{});
}


pub async fn receive<T>( 
    mut rx:Receiver<ProviderChannel<T>>,
    mut endpoint:Box<dyn EndPoint<T> + Send>
) -> bool {
    
    let data_full = rx.recv().await.unwrap();

    let _data:T;
    match data_full {
        ProviderChannel::Complete => {
            println!("First Batch Required");
        },
        ProviderChannel::Data(x) => {
            _data = x;
        },
        ProviderChannel::Info(_) => {},
    }
    
    // Wait for the rest of the inputs to flush out to exit
    let mut passed = true;
    loop {
        let result = rx.recv().await; //.unwrap();
        
        match result {
            Some(ProviderChannel::Info(_)) => {
                continue;
            }   
            Some(ProviderChannel::Complete) => {
                println!("Done Receiver");
                return passed;
            },
            Some(ProviderChannel::Data(data)) => {
                passed &= endpoint.receive(data);
            },
            None => {
                println!("RX ERROR");
                return passed & false;
            }
        }
    }
}

pub async fn create_endpoint<D:Serialize+Send+'static>(value:Arc<serde_yaml::Value>,
    endpoint:Box<dyn Fn(&Arc<serde_yaml::Value>) -> Box<dyn EndPoint<D> + Send>>,
    rx:tokio::sync::mpsc::Receiver<ProviderChannel<D>>) -> JoinHandle<bool> {
    // Create the Data Provider
    let endpoint = endpoint(&value.clone());

    let handle = task::spawn(async move {
        let result = receive(rx, endpoint);
        result.await
            
    });  
    handle
}