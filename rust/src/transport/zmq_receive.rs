


use serde::{Deserialize, Serialize};

use std::time::{Instant};
use std::process::{Command};

#[derive(Debug,Deserialize,Serialize, Clone)]
pub struct PythonCommand {
    pub command:String,
    pub cwd:String,
    pub args:Vec<String>
}

#[derive(Debug,Deserialize,Serialize, Clone)]
pub enum NodeConfig{
    #[serde(rename = "python")]
    Python(PythonCommand),
    #[serde(rename = "none")]
    None
}



// Endpoint to Accomdate Testing where data is received and counted
pub async fn rust_node_transport<'de,T:Deserialize<'de>>(address:String, batch_size:u64) -> bool {
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
        
        match serde_pickle::from_slice::<T>(bytes.as_slice(), Default::default()) {
            Ok(_x) => {count += 1}
            Err(_) => {
                break;
            },
        }
    }
    let time = now.elapsed().as_micros() as f32/1000000.0;
    let batches = count*batch_size;
    let qps = (batches as f32)/(time as f32);
    log::info!("Batches {}, RunTime {}s, QPS {}",batches, time, qps);
    true
}

// Wrapper around running a python version of the model
pub async fn python_node_transport(python_command:PythonCommand) -> bool  {
    println!("Running {} {} {:?}", python_command.command, python_command.cwd, python_command.args);
    let result = Command::new(python_command.command).current_dir(python_command.cwd).args(python_command.args).output();
    let stderr = &result.unwrap().stdout[..]; 
    let std_string = std::str::from_utf8(stderr).unwrap();
    let fail = std_string.find("FAILED");

    log::info!("Stdout {}", std_string);
    match fail {
        Some(_) => false,
        None => true
    }
    //log::info!("Stderr {}", &result.unwrap().stderr);

}

pub async fn dummy_node_tranport() -> bool  {
    return true;

}