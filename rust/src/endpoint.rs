
use std::sync::Arc;

use serde::Serialize;
use tokio::task::{JoinHandle, self};

use crate::{transport::ZmqChannel};

pub trait EndPoint<T> {
    fn receive(&mut self, data:T) -> bool;
}



pub async fn receive<T>( 
    mut rx:tokio::sync::mpsc::Receiver<ZmqChannel<T>>,
    mut endpoint:Box<dyn EndPoint<T> + Send>
) -> bool {
    
    let data_full = rx.recv().await.unwrap();

    let _data:T;
    match data_full {
        ZmqChannel::Complete => {
            println!("First Batch Required");
            //_data = SquadData::new(1, 1);
        },
        ZmqChannel::Data(x) => {
            _data = x;
        },
    }
    
    // Wait for the rest of the inputs to flush out to exit
    loop {
        let result = rx.recv().await; //.unwrap();
        match result {
            Some(ZmqChannel::Complete) => {
                println!("Done Receiver");
                return true;
            },
            Some(ZmqChannel::Data(data)) => {
                return endpoint.receive(data);
                //println!("RX");    
            },
            None => {
                println!("RX ERROR");
                return true;
            }
        }
    }
}

pub async fn create_endpoint<D:Serialize+Send+'static>(value:Arc<serde_yaml::Value>,
    endpoint:Box<dyn Fn(&Arc<serde_yaml::Value>) -> Box<dyn EndPoint<D> + Send>>,
    rx:tokio::sync::mpsc::Receiver<ZmqChannel<D>>) -> JoinHandle<bool> {
    // Create the Data Provider
    let endpoint = endpoint(&value.clone());

    let handle = task::spawn(async move {
        let result = receive(rx, endpoint);
        result.await
            
    });  
    handle
}