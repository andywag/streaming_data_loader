



use serde::Serialize;
use tokio::sync::mpsc::Receiver;
use tokio::sync::mpsc::{Sender};
use tokio::task::{self, JoinHandle};


use crate::batcher::BatchConfig;
use crate::batcher::{self, Batcher};
use crate::config::ModelType;
use crate::config::TrainingConfig;
use crate::datasets::dataset::DataSet;
use crate::datasets::dataset_config::DataSetConfig;
use crate::tokenizer::tokenizer_config::TokenizerInternalConfig;
use crate::tokenizer::tokenizer_wrapper;
use crate::tokenizer::tokenizer_wrapper::TokenizerWrapper;
use crate::transport::test_endpoint::{EndPoint};
use crate::provider::provider_config::ProviderConfig;
use crate::provider::arrow_transfer::{ArrowTransfer};
use crate::provider::{ProviderChannel};
use crate::transport::zmq_receive::NodeConfig;
use crate::transport::{self};


pub async fn create_data_provider<P:Clone + Send + 'static>(provider_config:ProviderConfig,
    dataset_config:DataSetConfig,
    provider:Box<dyn Fn(&ProviderConfig, DataSetConfig) -> ArrowTransfer<P>>,
    tx:tokio::sync::mpsc::Sender<ProviderChannel<P>>
    ) -> JoinHandle<()> {

    // Create the Provider Configuration

    let mut loader = provider(&provider_config, dataset_config);
    let join_provider = task::spawn(async move {    
        let load_result = loader.load_data(provider_config, tx);
        load_result.await;
    });
    join_provider
}

pub async fn create_tokenizer<P:Send + 'static, D:Serialize+Send+'static>(
    tokenizer_config:TokenizerInternalConfig,
    model_type:ModelType,
    config_batch:BatchConfig,
    dataset_config:DataSetConfig,
    generator:Box<dyn Fn(ModelType, BatchConfig, DataSetConfig, TokenizerWrapper)-> Box<dyn Batcher<S=P,T=D> + Send>>,
    rx:Receiver<ProviderChannel<P>>, 
    tx:Sender<ProviderChannel<D>>) -> JoinHandle<()> {
    // Create the Data Provider
    
    
    let tokenizer = tokenizer_wrapper::get_tokenizer(tokenizer_config).unwrap();
    let generator = generator(model_type, config_batch, dataset_config, tokenizer);

    let join_tokenizer = task::spawn(async move {
        let result = batcher::create_batch(rx, tx, generator);
        result.await;
    });
    join_tokenizer
}



pub enum ProviderType<L,R> {
    Sync(L),
    Async(R)
}

type DataProviderAsync<P> = Box<dyn Fn(ProviderConfig, Sender<ProviderChannel<P>>, Option<String>) -> JoinHandle<()>>;
type DataProviderSync<P> = Box<dyn Fn(&ProviderConfig, DataSetConfig) -> ArrowTransfer<P>>;

// TODO : Clean up the direct reading of the Serde Value and use a serde load to a struct
pub async fn run_main<'de, P:Clone + Send + 'static>(
    config:TrainingConfig,
    base_provider:ProviderType<DataProviderAsync<P>,DataProviderSync<P>>,
    generator:Box<dyn Fn(ModelType, BatchConfig, DataSetConfig, TokenizerWrapper)-> Box<dyn Batcher<S=P,T=DataSet> + Send>>,
    _endpoint:Box<dyn Fn(TrainingConfig) -> Box<dyn EndPoint<DataSet> + Send>>,
    cache:Option<String>
    ) -> bool {


    let config_copy = config.clone();

    // Create the Channel from Input to Tokenizer
    let (tx, rx) = tokio::sync::mpsc::channel::<ProviderChannel<P>>(2);
    // Create the Channel from Tokenizer to Output
    let (tx_trans, rx_trans) = tokio::sync::mpsc::channel::<ProviderChannel<DataSet>>(1);

    // Data Loading Configuration

    // Create the Data Provider Configuration
    let join_provider = match base_provider {
        ProviderType::Sync(x) => x(config.source, tx, cache),
        ProviderType::Async(y) => create_data_provider(config.source, config.dataset_config.clone(),y, tx).await,
    };

    // Create the batcher
    let join_tokenizer = create_tokenizer(config.tokenizer,
        config.model_config,  
        config.batch, 
        config.dataset_config, 
        generator, 
        rx, 
        tx_trans);

    // Create One of 2 Options 
    // 1. "test" : Create an internal test endpoint
    // 2. ""     : Create a zmq endpoint which talks to external process

    let join_rx = transport::create_transport(config_copy, rx_trans).await;
    /* 
    let join_rx = match config.transport.transport{
        transport::TransportEnum::Test => {
            let endpoint = endpoint(config_copy);

            task::spawn(async move {
                let result = test_endpoint::receive(rx_trans, endpoint);
                result.await
            
            })   
        },
        transport::TransportEnum::Zmq{address} => {
            task::spawn(async move {
                let result = transport::zmq_transmit::receive_transport(address, rx_trans, training_config);
                result.await
            })
        },
        
    };
    */

    // Create one of 2 options 
    // 1. "none"   : No Operation with either the test mode 
    // 2. "python" : External Python Command

    let join_node = match config.node {
        NodeConfig::Python(config) => { // Python Option
            task::spawn(async move {
                let result = transport::zmq_receive::python_node_transport(config);
                result.await
            })
        },
        NodeConfig::None => { // Bypass Option
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




