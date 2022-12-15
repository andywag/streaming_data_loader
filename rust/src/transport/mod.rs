

use std::sync::mpsc::SyncSender;

use serde::{Deserialize, Serialize};
use futures::{FutureExt};
use tokio::{sync::mpsc::Receiver, task::{self, JoinHandle}};

use crate::{provider::{ProviderChannel,}, tasks::{masking::masking_test_endpoint::MaskingEndpoint, python::python_cases}, datasets::dataset::DataSet, config::TrainingConfig};

use self::zmq_receive::{NodeConfig, PythonCommand};




pub mod zmq_transmit;
pub mod zmq_receive;
pub mod test_endpoint;
pub mod channel_transmit;

#[derive(Deserialize, Serialize, Debug, Clone)]

pub enum TransportEnum {
    #[serde(rename = "test")]
    Test,
    #[serde(rename = "zmq")]
    Zmq{address:String},
    //#[serde(rename = "channel")]
    //Channel
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TransportConfig {
    pub transport:TransportEnum
}



pub async fn create_transport(config:TrainingConfig, 
    mut rx_trans:Receiver<ProviderChannel<DataSet>>, 
    destination:Option<SyncSender<ProviderChannel<DataSet>>>) -> JoinHandle<bool>  {

    let transport_config = config.transport.transport.clone();
    //type D = u32;
    let test_endpoint = |x: Receiver<ProviderChannel<DataSet>> | async move { 
        task::spawn(async move {
            let endpoint = Box::new(MaskingEndpoint::new(python_cases::get_case(python_cases::Cases::Basic, true)));
            test_endpoint::receive(x, endpoint).await
        })
    }.boxed();

    let test_transport_endpoint = |x: Receiver<ProviderChannel<DataSet>> | async move { 
        task::spawn(async move {
            match &config.transport.transport {
                TransportEnum::Test => todo!(),
                TransportEnum::Zmq { address } => {
                    let result = zmq_transmit::receive_transport(address.clone(), x, config.clone());
                    result.await
                },
            }
            
        })
    }.boxed();

    if destination.is_some() {
        let tx = destination.unwrap();
        task::spawn(async move {
            loop {
                let data = rx_trans.recv().await;
                match data {
                    Some(ProviderChannel::Complete) => {let _ = tx.send(ProviderChannel::Complete); break;}
                    Some(ProviderChannel::Data(x)) => {
                        let result = tx.send(ProviderChannel::Data(x));
                        if result.is_err() {
                            log::error!("Transmit Failed");
                        }
                    }
                    Some(ProviderChannel::Info(_x)) => {},
                    _ => {log::error!("Failed Data");},
                };
            }
            true  
        })
    }
    else {
        let result = match transport_config {
            TransportEnum::Test => test_endpoint(rx_trans),
            TransportEnum::Zmq { address:_ } => test_transport_endpoint(rx_trans),
        };
        result.await
    }


    
    
    
}


pub async fn create_transport_node(config:NodeConfig) -> JoinHandle<bool>  {

    //type D = u32;
    let python_transport = |command:PythonCommand | async move { 
        task::spawn(async move {
            let result = zmq_receive::python_node_transport(command);
            result.await
        })
    }.boxed();

    let dummy_transport = || async move { 
        task::spawn(async move {
            let result = zmq_receive::dummy_node_tranport();
            result.await
        })
    }.boxed();

    let result = match config {
        NodeConfig::Python(command) => python_transport(command),
        NodeConfig::None => dummy_transport(),
    };
    result.await
    
    
}

