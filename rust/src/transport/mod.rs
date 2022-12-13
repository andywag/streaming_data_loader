

use serde::{Deserialize, Serialize};
use futures::{FutureExt};
use tokio::{sync::mpsc::Receiver, task::{self, JoinHandle}};

use crate::{provider::{ProviderChannel,}, tasks::{masking::masking_test_endpoint::MaskingEndpoint, python::python_cases}, datasets::dataset::DataSet, config::TrainingConfig};




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



pub async fn create_transport(config:TrainingConfig, rx_trans:Receiver<ProviderChannel<DataSet>>) -> JoinHandle<bool>  {

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

    let result = match transport_config {
        TransportEnum::Test => test_endpoint(rx_trans),
        TransportEnum::Zmq { address:_ } => test_transport_endpoint(rx_trans),
    };
    result.await
    
    
}

