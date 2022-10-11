

use serde::Serialize;
use tokio::sync::mpsc::Receiver;
//use tokio::sync::mpsc::Sender;

// Creat the serverer function
pub async fn receieve_transport<T:Serialize>(_host:String, port:u64, mut rx:Receiver<T>) -> bool {
    let ctx = zmq::Context::new();
    
    //let host_name = format!("tcp://{}:{}", host, port);
    let host_name = format!("tcp://*:{}", port);

    println!("Starting Server at {}", host_name);
    let socket = ctx.socket(zmq::REP).unwrap();
    socket.bind(host_name.as_str()).unwrap();

    loop {
        let mut msg = zmq::Message::new();
        let _ = socket.recv(&mut msg, 0);
        let data = rx.recv().await;
        let result = serde_pickle::to_vec(&data.unwrap(), Default::default());
        let _ = socket.send(result.unwrap(), 0);
    }
    //true
}


