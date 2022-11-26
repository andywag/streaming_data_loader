


use std::sync::Arc;

use serde::Serialize;
use serde::Deserialize;
use serde_yaml::Value;
use tokio::sync::mpsc::Receiver;
use tokio::sync::mpsc::{Sender};
use tokio::task::{self, JoinHandle};


use crate::batcher::{self, Batcher};
use crate::tokenizer_wrapper;
use crate::tokenizer_wrapper::TokenizerWrapper;
use crate::transport::test_endpoint::{self, EndPoint};
use crate::provider::ProviderConfig;
use crate::provider::arrow_transfer::{ArrowTransfer};
use crate::provider::{ProviderChannel};
use crate::transport::zmq_receive::NodeConfig;
use crate::transport::{self};
use crate::transport::TransportConfig;

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
    generator:Box<dyn Fn(&Arc<serde_yaml::Value>, TokenizerWrapper)-> Box<dyn Batcher<S=P,T=D> + Send>>,
    rx:Receiver<ProviderChannel<P>>, 
    tx:Sender<ProviderChannel<D>>) -> JoinHandle<()> {
    // Create the Data Provider
    
    let tokenizer_name = value["tokenizer"]["name"].as_str().unwrap().to_string().to_owned();
    let tokenizer = tokenizer_wrapper::get_tokenizer(tokenizer_name).unwrap();
    let generator = generator(&value, tokenizer);

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
    generator:Box<dyn Fn(&Arc<serde_yaml::Value>, TokenizerWrapper)-> Box<dyn Batcher<S=P,T=D> + Send>>,
    endpoint:Box<dyn Fn(&Arc<serde_yaml::Value>) -> Box<dyn EndPoint<D> + Send>>) -> bool {

    // Create the Channel from Input to Tokenizer
    let (tx, rx) = tokio::sync::mpsc::channel::<ProviderChannel<P>>(2);
    // Create the Channel from Tokenizer to Output
    let (tx_trans, rx_trans) = tokio::sync::mpsc::channel::<ProviderChannel<D>>(1);

    // Create the Data Provider Configuration
    let join_provider = match base_provider {
        Either::Left(x) => x(&value.clone(), tx),
        Either::Right(y) => create_data_provider(value.clone(), y, tx).await,
    };

    // Create the batcher
    let join_tokenizer = create_tokenizer(value.clone(), generator, rx, tx_trans);

    // Create One of 2 Options 
    // 1. "test" : Create an internal test endpoint
    // 2. ""     : Create a zmq endpoint which talks to external process
    let transport_config = serde_yaml::from_value::<TransportConfig>(value["transport"].clone()).unwrap();
    let join_rx = match transport_config.transport {
        transport::TransportEnum::Test => {
            let endpoint = endpoint(&value.clone());

            task::spawn(async move {
                let result = test_endpoint::receive(rx_trans, endpoint);
                result.await
            
            })   
        },
        transport::TransportEnum::Zmq{address} => {
            task::spawn(async move {
                let result = transport::zmq_transmit::receive_transport(address, rx_trans);
                result.await
            })
        },
    };

    // Create one of 2 options 
    // 1. "none"   : No Operation with either the test mode 
    // 2. "python" : External Python Command
    //let node_select = value["node"]["type"].as_str().unwrap();

    let node_config = serde_yaml::from_value::<NodeConfig>(value["node"].clone());
    let join_node = match node_config {
        Result::Ok(NodeConfig::Python(config)) => { // Python Option
            task::spawn(async move {
                let result = transport::zmq_receive::python_node_transport(config);
                result.await
            })
        },
        Result::Ok(NodeConfig::None) => { // Bypass Option
            task::spawn(async move {
                let result = transport::zmq_receive::dummy_node_tranport();
                result.await
            })
        },
        Result::Err(e) => { // Error
            // TODO : Crash Run rather than continue operations
            log::error!("Error Decoding configuration {:?}", e);
            
            task::spawn(async move {
                let result = transport::zmq_receive::dummy_node_tranport();
                result.await
            })
        }
    };
    


    let result = tokio::join!(join_rx, join_tokenizer, join_provider, join_node);
    log::info!("Finished : Internal-{:?} External-{:?}", result.0, result.3);
    return result.0.unwrap() && result.3.unwrap();
    
    
}




