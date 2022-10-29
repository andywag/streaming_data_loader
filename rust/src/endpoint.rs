
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