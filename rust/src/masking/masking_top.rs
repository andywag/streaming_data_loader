


use serde_yaml::Value;
use tokio::task::{self};

use super::masking_config::MaskingConfig;
use super::{masking::BaseTokenizer, masked_data::MaskedData};
use super::super::provider;
use crate::{utils};
use crate::transport;

pub async fn create_tokenizer(config:&MaskingConfig, rx:tokio::sync::mpsc::Receiver<String>, 
    tx_transport:tokio::sync::mpsc::Sender<MaskedData>) {
    let base_tokenizer = BaseTokenizer::new(config);
    
    let result = base_tokenizer.create_batch(rx, tx_transport);
    result.await;
}

pub async fn provider(location:String, network:bool, tx:tokio::sync::mpsc::Sender<String>)  {
    if network { // URL to web version of file
        provider::wiki_file_provider::load_url(&location, tx).await;
    }
    else {// Downloaded File
        provider::wiki_file_provider::load_data(&location, tx).await;
    }
}


pub async fn receiver(config_in:&MaskingConfig, 
    mut rx:tokio::sync::mpsc::Receiver<MaskedData>,
    compare_location:Option<String>
) -> bool {
    let config = config_in.clone();
    let mut data = rx.recv().await.unwrap();
    //println!("Data {:?}", data);

        //let mut result = data.clone();
    for x in 0..config.batch_size as usize {
        for y in 0..config.mask_length as usize {
            data.input_ids[x][data.masked_lm_positions[x][y] as usize] = data.masked_lm_labels[x][y];
        }
    }
    match compare_location {
        Some(path) => {
            let check = utils::compare_data(path, data.input_ids, 256);
            println!("Match {check}");
            return check;
        },
        None => return true,
    }
    
}

pub async fn run_main(value:&Value) {
    let (tx, rx) = tokio::sync::mpsc::channel::<String>(2);
    let (tx_trans, rx_trans) = tokio::sync::mpsc::channel::<MaskedData>(2);

    let tokenizer = &value["tokenizer"]["config"];
    let config:MaskingConfig = serde_yaml::from_value(tokenizer.to_owned()).unwrap();

    let loc = &value["source"]["location"];
    let location:String = serde_yaml::from_value(loc.to_owned()).unwrap();

    //let compare_loc = &value["sink"]["config"]["comparison"]; 
    //let compare_location:Option<String> = serde_yaml::from_value(compare_loc.to_owned()).ok();

    //println!("Compare {:?}", compare_location);

    //utils::get_tokenizer("bert-base-uncased".to_string());
    //let base = "https://dumps.wikimedia.org/other/cirrussearch/current/";
    //let location = "/home/andy/Downloads/enwiki-20220926-cirrussearch-content.json.gz".to_string();
    //let config = MaskingConfig{batch_size:8, sequence_length:128, mask_length:18, tokenizer_name:"bert-base-uncased".to_string()};
    let config2 = config.clone();
    //let location = "../data/test.json.gz".to_string();

    //}


    let rx_select = value["sink"]["type"].as_str().map(|e| e.to_string());
    //let join_rx = task::spawn(async move {
        
    let join_rx = if rx_select.unwrap() == "mask_receiver" {
        let compare_location = value["sink"]["config"]["comparison"].as_str().map(|e| e.to_string());
        task::spawn(async move {
            let result = receiver(&config, rx_trans, compare_location);
            result.await
        })   
    }
    else {
        let host = value["sink"]["config"]["host"].as_str().unwrap().to_string();
        let port = value["sink"]["config"]["port"].as_u64().unwrap();
        task::spawn(async move {
            let result = transport::zmq_transport::receieve_transport(host, port, rx_trans);
            result.await
        })
    };

        
    let join_tokenizer = task::spawn(async move {
        let tok = create_tokenizer(&config2, rx, tx_trans);
        tok.await;
    });

    let join_provider = task::spawn(async move {
        let base = provider(location, false, tx);
        base.await;
    });

    //let total = tokio::join!(join_rx, join_tokenizer, join_provider);

    tokio::select! {        
        _ = join_provider => {}
        _ = join_tokenizer => {}
        _ = join_rx => {
            println!("Exiting");
            //assert!(rx.unwrap());
            std::process::exit(0);
        }
    };
}