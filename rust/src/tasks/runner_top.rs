


use std::sync::Arc;

use serde::Serialize;
use serde::Deserialize;
use serde_yaml::Value;
use tokio::sync::mpsc::{Sender};
use tokio::task::{self, JoinHandle};


use crate::batcher::{self, Batcher};
use crate::test_endpoint::{self, EndPoint};
use crate::provider::ProviderConfig;
use crate::provider::arrow_transfer::{ArrowTransfer};
use crate::provider::{ProviderChannel};
use crate::transport::{self};

pub async fn create_data_provider<P:Clone + Send + 'static>(value:Arc<Value>, 
    provider:Box<dyn Fn(&ProviderConfig) -> ArrowTransfer<P>>,
    tx:tokio::sync::mpsc::Sender<ProviderChannel<P>>
    ) -> JoinHandle<()> {

    // Create the Provider Configuration
    let provider_config = serde_yaml::from_value::<ProviderConfig>(value["source"].clone()).unwrap();

    let mut loader = provider(&provider_config);
    let join_provider = task::spawn(async move {    
        let load_result = loader.load_data(provider_config, tx);
        load_result.await;
    });
    join_provider
}

pub async fn create_tokenizer<P:Send + 'static, D:Serialize+Send+'static>(value:Arc<serde_yaml::Value>,
    generator:Box<dyn Fn(&Arc<serde_yaml::Value>)-> Box<dyn Batcher<S=P,T=D> + Send>>,
    rx:tokio::sync::mpsc::Receiver<ProviderChannel<P>>, 
    tx:tokio::sync::mpsc::Sender<ProviderChannel<D>>) -> JoinHandle<()> {
    // Create the Data Provider
    let generator = generator(&value);

    let join_tokenizer = task::spawn(async move {
        let result = batcher::create_batch(rx, tx, generator);
        result.await;
    });
    join_tokenizer
}

pub async fn create_endpoint<D:Serialize+Send+'static>(value:Arc<serde_yaml::Value>,
    endpoint:Box<dyn Fn(&Arc<serde_yaml::Value>) -> Box<dyn EndPoint<D> + Send>>,
    rx:tokio::sync::mpsc::Receiver<ProviderChannel<D>>) -> JoinHandle<bool> {
    // Create the Data Provider
    let endpoint = endpoint(&value.clone());

    let handle = task::spawn(async move {
        let result = test_endpoint::receive(rx, endpoint);
        result.await
            
    });  
    handle
}

pub enum Either<L,R> {
    Left(L),
    Right(R)
}

type DataProviderAsync<P> = Box<dyn Fn(&Arc<Value>, Sender<ProviderChannel<P>>) -> JoinHandle<()>>;
type DataProviderSync<P> = Box<dyn Fn(&ProviderConfig) -> ArrowTransfer<P>>;

// TODO : Clean up the direct reading of the Serde Value and use a serde load to a struct
pub async fn run_main<'de, P:Clone + Send + 'static, D:Deserialize<'de>+Serialize+Send+'static>(value:Arc<Value>,
    base_provider:Either<DataProviderAsync<P>,DataProviderSync<P>>,
    generator:Box<dyn Fn(&Arc<serde_yaml::Value>)-> Box<dyn Batcher<S=P,T=D> + Send>>,
    endpoint:Box<dyn Fn(&Arc<serde_yaml::Value>) -> Box<dyn EndPoint<D> + Send>>) -> bool {


    // Create the Channel from Input to Tokenizer
    let (tx, rx) = tokio::sync::mpsc::channel::<ProviderChannel<P>>(2);
    // Create the Channel from Tokenizer to Output
    let (tx_trans, rx_trans) = tokio::sync::mpsc::channel::<ProviderChannel<D>>(1);

    // Create the Data Provider Configuration
    //let provider_config = serde_yaml::from_value::<ProviderConfig>(value["source"].clone()).unwrap();

    let join_provider = match base_provider {
        Either::Left(x) => x(&value.clone(), tx),
        Either::Right(y) => create_data_provider(value.clone(), y, tx).await,
    };

    let join_tokenizer = create_tokenizer(value.clone(), generator, rx, tx_trans);



    // Create the Receiver : Either a test endpoint for local testing or a ZMQ transport for an external operation
    let rx_select = value["sink"]["type"].as_str().map(|e| e.to_string());
    let join_rx = if rx_select.unwrap() == "test" { // Local Test Point
        let endpoint = endpoint(&value.clone());

        task::spawn(async move {
            let result = test_endpoint::receive(rx_trans, endpoint);
            result.await
            
        })   
    }
    else { // Send to Processing Node
        let address = value["sink"]["config"]["address"].as_str().unwrap().to_string();        
        task::spawn(async move {
            let result = transport::zmq_transmit::receive_transport(address, rx_trans);
            result.await
        })
    };


    let node_select = value["node"]["type"].as_str().unwrap();
    
    // Option for no processing element for test
    if node_select == "none" { // Option where node point
        println!("Creating without Sink Node");
        let result = tokio::join!(join_rx, join_tokenizer, join_provider);
        println!("Finished {:?}", result.0);
        return true;
    }
    else {
        let join_node = {
            if node_select == "rust" {
                let address = value["sink"]["config"]["address"].as_str().unwrap().to_string();
                let batch_size = value["tokenizer"]["config"]["batch_size"].as_u64().unwrap();

                task::spawn(async move {
                    let result = transport::zmq_receive::rust_node_transport::<D>(address, batch_size);
                    result.await
                })
            }
            else if node_select == "python" {
                let command = value["node"]["config"]["cmd"].as_str().unwrap().to_string();
                let cwd = value["node"]["config"]["cwd"].as_str().unwrap().to_string();
                let args:Vec<String> = value["node"]["config"]["args"].as_sequence().unwrap().into_iter().map(|e|e.as_str().unwrap().to_string()).collect();
    
                task::spawn(async move {
                    let result = transport::zmq_receive::python_node_transport(command,cwd,args);
                    result.await
                })
            }
            else {
                let address = value["sink"]["config"]["address"].as_str().unwrap().to_string();
                let batch_size = value["tokenizer"]["config"]["batch_size"].as_u64().unwrap();

                task::spawn(async move {
                    let result = transport::zmq_receive::rust_node_transport::<D>(address, batch_size);
                    result.await
                })
            }

        };
        let result = tokio::join!(join_rx, join_tokenizer, join_provider, join_node);
        log::info!("Finished {:?} {:?}", result.0, result.3);
        return result.0.unwrap() && result.3.unwrap();
    }
    
    
}




