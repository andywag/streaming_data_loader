use std::sync::Arc;

use serde_yaml::Value;

pub mod wiki_file_provider;

pub enum ProviderChannel<T> {
    Complete,
    Data(T)
}

pub async fn create_provider<'a>(value:Arc<Value>, tx:tokio::sync::mpsc::Sender<ProviderChannel<String>>)  {
    
    let iterations = value["source"]["iterations"].as_u64().unwrap().to_owned();
    let location = value["source"]["location"].as_str().unwrap().to_string();
    let source_type = value["source"]["type"].as_str().unwrap().to_string();

    let network = source_type == "wiki_url";

    if network { // URL to web version of file
        wiki_file_provider::load_url(&location, iterations, tx).await;
    }
    else {// Downloaded File
        wiki_file_provider::load_data(&location, iterations, tx).await;
    }
}
/* 
pub async fn provider(location:String, network:bool, iterations:u64, tx:tokio::sync::mpsc::Sender<ProviderChannel<String>>)  {
    if network { // URL to web version of file
        wiki_file_provider::load_url(&location,  tx).await;
    }
    else {// Downloaded File
        wiki_file_provider::load_data(&location, iterations, tx).await;
    }
}
*/

