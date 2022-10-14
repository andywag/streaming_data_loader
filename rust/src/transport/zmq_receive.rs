


use crate::masking::masked_data::MaskedData;
use std::time::{Instant};
use std::process::{Command};


pub async fn rust_node_transport(address:String, batch_size:u64)  {
    let ctx = zmq::Context::new();
     
    let host_name = address;

    println!("Starting Receiver at {}", host_name);
    let socket = ctx.socket(zmq::REQ).unwrap();
    socket.connect(host_name.as_str()).unwrap();
    let now = Instant::now();

    // it prints '2'
    let mut count = 0;
    loop {
        let _ = socket.send("Hello", 0);
        let bytes = socket.recv_bytes(0).unwrap();
        
        match serde_pickle::from_slice::<MaskedData>(bytes.as_slice(), Default::default()) {
            Ok(_x) => {count += 1}
            Err(_) => {
                break;
            },
        }
    }
    let time = now.elapsed().as_micros() as f32/1000000.0;
    let batches = count*batch_size;
    let qps = (batches as f32)/(time as f32);
    println!("Batches {}, RunTime {}s, QPS {}",batches, time, qps);
}

pub async fn python_node_transport(command:String, cwd:String, args:Vec<String>)  {
    println!("Running {} {} {:?}", command, cwd, args);
    let output = Command::new(command).current_dir(cwd).args(args).output();
    let result = std::str::from_utf8(output.unwrap().stdout.as_slice()).unwrap().to_string();
    println!("{:?}", result);
}