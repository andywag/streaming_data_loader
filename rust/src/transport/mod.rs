use serde::{Deserialize, Serialize};


pub mod zmq_transmit;
pub mod zmq_receive;
pub mod test_endpoint;

#[derive(Deserialize, Serialize, Debug)]

pub enum TransportEnum {
    #[serde(rename = "test")]
    Test,
    #[serde(rename = "zmq")]
    Zmq{address:String}
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TransportConfig {
    pub transport:TransportEnum
   
}
