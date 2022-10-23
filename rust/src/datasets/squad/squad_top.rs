


use std::sync::Arc;

use serde_yaml::Value;
use tokio::task::{self};


use super::squad_data::{SquadGeneral, SquadData};
use super::{SquadConfig, squad_tokenizer, squad_endpoint, squad_arrow};
use crate::provider::{ProviderChannel, arrow_provider};
use crate::transport::{self, ZmqChannel};

pub async fn create_provider<'a>(value:Arc<Value>, tx:tokio::sync::mpsc::Sender<ProviderChannel<SquadGeneral>>)  {
    

    let iterations = value["source"]["iterations"].as_u64().unwrap().to_owned();
    let location = if value["source"]["type"] == "arrow" {
        let location = value["source"]["location"].as_str().unwrap().to_string();
        //let loader = squad_arrow::SquadArrowLoader::new(location);
        //loader.load_data(iterations, tx).await;
        location
    }
    else if value["source"]["type"] == "hugging" {
        let dataset = value["source"]["dataset"].as_str().unwrap().to_string();
        let arg = value["source"]["arg"].as_str();//.map(|e| e.to_string());
        let key = value["source"]["key"].as_str().unwrap().to_string();
        let arrow_file = arrow_provider::download_huggingface_dataset(dataset.as_str(), arg, key.as_str());
        let location = arrow_file.unwrap()[0].to_string();
        location
    }
    else {
        println!("Source Type Not Defined");
        "".to_string()
    };
    let loader = squad_arrow::SquadArrowLoader::new(location);
    loader.load_data(iterations, tx).await;
    /* 
    if value["source"]["type"] == "arrow" {
        let location = value["source"]["location"].as_str().unwrap().to_string();
        let loader = squad_arrow::SquadArrowLoader::new(location);
        loader.load_data(iterations, tx).await;
    }
    else if value["source"]["type"] == "hugging" {
        let dataset = value["source"]["dataset"].as_str().unwrap().to_string();
        let arg = value["source"]["arg"].as_str();//.map(|e| e.to_string());
        let key = value["source"]["key"].as_str().unwrap().to_string();
        let arrow_file = arrow_provider::download_huggingface_dataset(dataset.as_str(), arg, key.as_str());  
    }
    */
    

}

// TODO : The squad implementation has quite a few flaws and is not fully functional

pub async fn run_main(value:Arc<Value>) -> bool {
    let (tx, rx) = tokio::sync::mpsc::channel::<ProviderChannel<SquadGeneral>>(2);
    let (tx_trans, rx_trans) = tokio::sync::mpsc::channel::<ZmqChannel<SquadData>>(1);

    let tokenizer = &value["tokenizer"]["config"];
    let config:SquadConfig = serde_yaml::from_value(tokenizer.to_owned()).unwrap();


    let config_clone = config.clone();
    let provider_value = value.clone();

    // Create the Data Provider
    let join_provider = task::spawn(async move {
        //let base = provider::provider(location, false, iterations, tx);
        let base = create_provider(provider_value, tx);
        base.await;
    });

    // Create the tokenizer
    let join_tokenizer = task::spawn(async move {
        let tok = squad_tokenizer::create_tokenizer(&config_clone, rx, tx_trans);
        tok.await;
    });



    // Create the Receiver : Either a test endpoint or a zmq transport
    let rx_select = value["sink"]["type"].as_str().map(|e| e.to_string());
    let join_rx = if rx_select.unwrap() == "test" {
        task::spawn(async move {
            let result = squad_endpoint::receiver(&config, rx_trans);
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
        //return result.0.unwrap();
        return true;
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