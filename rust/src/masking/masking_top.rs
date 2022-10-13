


use serde_yaml::Value;
use tokio::task::{self};

use super::{masking, masking_endpoint};
use super::masking_config::MaskingConfig;
use super::{masked_data::MaskedData};
use super::super::provider;
use crate::provider::ProviderChannel;
use crate::transport::{self, ZmqChannel};






pub async fn run_main(value:&Value) -> bool {
    let (tx, rx) = tokio::sync::mpsc::channel::<ProviderChannel<String>>(2);
    let (tx_trans, rx_trans) = tokio::sync::mpsc::channel::<ZmqChannel<MaskedData>>(1);

    let tokenizer = &value["tokenizer"]["config"];
    let config:MaskingConfig = serde_yaml::from_value(tokenizer.to_owned()).unwrap();

    let loc = &value["source"]["location"];
    let location:String = serde_yaml::from_value(loc.to_owned()).unwrap();
    let iterations = value["source"]["iterations"].as_u64().unwrap().to_owned();

    //let compare_loc = &value["sink"]["config"]["comparison"]; 
    //let compare_location:Option<String> = serde_yaml::from_value(compare_loc.to_owned()).ok();

    //println!("Compare {:?}", compare_location);

    //utils::get_tokenizer("bert-base-uncased".to_string());
    //let base = "https://dumps.wikimedia.org/other/cirrussearch/current/";
    //let location = "/home/andy/Downloads/enwiki-20220926-cirrussearch-content.json.gz".to_string();
    //let config = MaskingConfig{batch_size:8, sequence_length:128, mask_length:18, tokenizer_name:"bert-base-uncased".to_string()};
    
    // Clone the config to pass to 2 different processes
    let config_clone = config.clone();

    //let location = "../data/test.json.gz".to_string();

    //}


    // Create the Data Provider
    let join_provider = task::spawn(async move {
        let base = provider::provider(location, false, iterations, tx);
        base.await;
    });

    // Create the tokenizer
    let join_tokenizer = task::spawn(async move {
        let tok = masking::create_tokenizer(&config_clone, rx, tx_trans);
        tok.await;
    });

    // Create the Receiver : Either a test endpoint or a zmq transport
    let rx_select = value["sink"]["type"].as_str().map(|e| e.to_string());
    let join_rx = if rx_select.unwrap() == "mask_receiver" {
        let compare_location = value["sink"]["config"]["comparison"].as_str().map(|e| e.to_string());
        task::spawn(async move {
            let result = masking_endpoint::receiver(&config, rx_trans, compare_location);
            result.await
        })   
    }
    else {
        let address = value["sink"]["config"]["address"].as_str().unwrap().to_string();        
        task::spawn(async move {
            let result = transport::zmq_transmit::receive_transport(address, rx_trans);
            result.await
        })
    };


    let node_select = value["node"]["type"].as_str().unwrap();
    
    if node_select == "none" { // Option where node point
        println!("Creating without Sink Node");
        let result = tokio::join!(join_rx, join_tokenizer, join_provider);
        println!("Finished {:?}", result.0);
        return result.0.unwrap();
    }
    else {
        let join_node = {
            if node_select == "rust" {
                let address = value["sink"]["config"]["address"].as_str().unwrap().to_string();
                let batch_size = value["tokenizer"]["config"]["batch_size"].as_u64().unwrap();

                task::spawn(async move {
                    let result = transport::zmq_receive::rust_node_transport(address, batch_size);
                    result.await
                })
            }
            else if node_select == "python" {
                let command = value["node"]["config"]["cmd"].as_str().unwrap().to_string();
                let cwd = value["node"]["config"]["cwd"].as_str().unwrap().to_string();
                let args:Vec<String> = value["node"]["config"]["args"].as_sequence().unwrap().into_iter().map(|e|e.as_str().unwrap().to_string()).collect();
    
                task::spawn(async move {
                    let result = transport::zmq_receive::python_node_transport(command,cwd,args);
                    result.await
                })
            }
            else {
                let address = value["sink"]["config"]["address"].as_str().unwrap().to_string();
                let batch_size = value["tokenizer"]["config"]["batch_size"].as_u64().unwrap();

                task::spawn(async move {
                    let result = transport::zmq_receive::rust_node_transport(address, batch_size);
                    result.await
                })
            }

        };
        let result = tokio::join!(join_rx, join_tokenizer, join_provider, join_node);
        println!("Finished {:?} {:?}", result.0, result.3);
        return result.0.unwrap();
    }

    
    //let total = tokio::join!(join_rx, join_tokenizer, join_provider);
    
}