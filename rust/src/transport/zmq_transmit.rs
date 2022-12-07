


use serde::{Serialize};
use tokio::sync::mpsc::Receiver;

use crate::{provider::ProviderChannel, config::TrainingConfig};





// Generic ZMQ Transfer to Send Data to Device
pub async fn receive_transport<T:Serialize>(address:String, 
    mut rx:Receiver<ProviderChannel<T>>,
    training_config:TrainingConfig) -> bool {

    let ctx = zmq::Context::new();
    
    let result:Vec<&str> = address.split(":").collect();
    let host_name = if result[0] == "tcp" {
        format!("tcp://*:{}", result[2])
    }
    else {
        address
    };
    

    log::info!("Starting Server on Connection :  {}", host_name);
    let socket = ctx.socket(zmq::REP).unwrap();
    socket.bind(host_name.as_str()).unwrap();


    let data = rx.recv().await;

    let dataset_info = if let ProviderChannel::Info(x) = data.unwrap() {
        println!("Info {:?}", x);
        Some(x)
    }
    else {
        log::error!("DataSet Info Required");
        None
    };

    let mut packet_count = 0;
    loop {
        let mut msg = zmq::Message::new();
        let _ = socket.recv(&mut msg, 0);
        
        match msg.as_str() {
            Some("Config") => {
                let result = serde_pickle::to_vec(&training_config, Default::default());
                let _ = socket.send(result.unwrap(), 0);
            }
            Some("Info") => {
                let result = serde_pickle::to_vec(&dataset_info, Default::default());
                let _ = socket.send(result.unwrap(), 0);
            },
            Some("Data") => {
                let data = rx.recv().await;
                match data.unwrap() {
                    ProviderChannel::Info(x) => {
                        println!("Getting Dataset Information {:?}", x);
                    }
                    ProviderChannel::Complete => {
                        log::info!("Finished Transport");
                        let _ = socket.send("Finished", 0);

                        return true;
                    },
                    ProviderChannel::Data(x) => {
                        let result = serde_pickle::to_vec(&x, Default::default());
                        let _ = socket.send(result.unwrap(), 0);
                        packet_count += 1;
                        if packet_count % 64 == 1 {
                            log::info!("Sent Packet {:?}", packet_count);
                        }
                    },
                }
            }
            _ => {
                log::error!("ZMQ Connection Error {:?}", msg);
            }
        }

    }
    //true
}




