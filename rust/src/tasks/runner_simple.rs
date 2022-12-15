



use std::sync::mpsc::SyncSender;

use serde::Serialize;
use tokio::sync::mpsc::Receiver;
use tokio::sync::mpsc::{Sender};
use tokio::task::{self, JoinHandle};


use crate::batcher::{self, Batcher};
use crate::config::TrainingConfig;
use crate::datasets::dataset::DataSet;
use crate::datasets::dataset_config::DataSetConfig;

use crate::provider::provider_config::ProviderConfig;
use crate::provider::arrow_transfer::{ArrowTransfer};
use crate::provider::{ProviderChannel};
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

pub async fn create_tokenizer<P:Send + 'static, D:Serialize+Send+'static>(config:TrainingConfig,
    generator:Box<dyn Fn(TrainingConfig)-> Box<dyn Batcher<S=P,T=D> + Send>>,
    rx:Receiver<ProviderChannel<P>>, 
    tx:Sender<ProviderChannel<D>>) -> JoinHandle<()> {
    // Create the Data Provider
    
    
    //let tokenizer = tokenizer_wrapper::get_tokenizer(config.tokenizer).unwrap();
    let generator = generator(config);

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
    generator:Box<dyn Fn(TrainingConfig)-> Box<dyn Batcher<S=P,T=DataSet> + Send>>,
    destination:Option<SyncSender<ProviderChannel<DataSet>>>,
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
    let join_tokenizer = create_tokenizer(config_copy.clone(),
        generator, 
        rx, 
        tx_trans);



    // Create the code which will transport the data to the end device
    let join_rx = transport::create_transport(config_copy, rx_trans, destination).await;
    // Optionally create the device
    let join_node = transport::create_transport_node(config.node).await;



    let result = tokio::join!(join_rx, join_tokenizer, join_provider, join_node);
    log::info!("Finished : Internal-{:?} External-{:?}", result.0, result.3);
    return result.0.unwrap() && result.3.unwrap();
    
    
}




