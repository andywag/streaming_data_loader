

use serde::{Serialize};
use tokio::sync::mpsc::Receiver;



use super::ZmqChannel;

//use tokio::sync::mpsc::Sender;

// Creat the serverer function
pub async fn receive_transport<T:Serialize>(address:String, mut rx:Receiver<ZmqChannel<T>>) -> bool {
    let ctx = zmq::Context::new();
    
    let result:Vec<&str> = address.split(":").collect();
    let host_name = if result[0] == "tcp" {
        format!("tcp://*:{}", result[2])
    }
    else {
        address
    };
    

    println!("Starting Server at {}", host_name);
    let socket = ctx.socket(zmq::REP).unwrap();
    socket.bind(host_name.as_str()).unwrap();

    loop {
        let mut msg = zmq::Message::new();
        let _ = socket.recv(&mut msg, 0);
        let data = rx.recv().await;
        match data.unwrap() {
            ZmqChannel::Complete => {
                println!("Finished Transport");
                let _ = socket.send("Finished", 0);
                return true;
            },
            ZmqChannel::Data(x) => {
                let result = serde_pickle::to_vec(&x, Default::default());
                let _ = socket.send(result.unwrap(), 0);
            },
        }
        
    }
    //true
}




