


use std::sync::mpsc::SyncSender;

use tokio::sync::mpsc::Receiver;

use crate::{provider::{ProviderChannel}};





// Generic ZMQ Transfer to Send Data to Device
pub async fn transport<DataSet>(tx:SyncSender<Option<DataSet>>, 
    mut rx:Receiver<ProviderChannel<DataSet>>) -> bool {


    loop {
    let data = rx.recv().await;

        match data {
            Some(ProviderChannel::Complete) => {
                let _ = tx.send(None);
            },
            Some(ProviderChannel::Info(_)) => {},
            Some(ProviderChannel::Data(x)) => {
                let _ = tx.send(Some(x));
            }
            None => {
                log::error!("Channel Reception Failed");
            },
        };
    }

   
    
    //true
}




