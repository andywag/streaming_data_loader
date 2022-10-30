use std::sync::Arc;

use serde_yaml::Value;
use tokio::task::{JoinHandle, self};

use crate::{provider::{ProviderChannel, wiki_file_provider}, datasets::generic_runner};
use tokio::sync::mpsc::Sender;

use super::{masking_tokenizer, masking_config::MaskingConfig, masked_data::MaskedData, masking_endpoint::MaskingEndpoint};




// Create the Dataset Provider for Squad
fn create_provider(value:&Arc<Value>, tx:Sender<ProviderChannel<String>>) -> JoinHandle<()> {
    let iterations = value["source"]["iterations"].as_u64().unwrap().to_owned();
    let location = value["source"]["location"].as_str().unwrap().to_string();
    let source_type = value["source"]["type"].as_str().unwrap().to_string();

    let network = source_type == "wiki_url";
    let handle = task::spawn(
        async move {
        if network { // URL to web version of file
            wiki_file_provider::load_url(&location, iterations, tx).await
        }
        else {// Downloaded File
            wiki_file_provider::load_data(&location, iterations, tx).await
        }
    });
    
    handle
}

 
// Create the Tokenizer for Squad
fn create_generator(value:&Arc<serde_yaml::Value>)-> Box<dyn crate::batcher::Batcher<S=String,T=MaskedData> + Send> {
    let tokenizer = &value["tokenizer"]["config"];
    let config:MaskingConfig = serde_yaml::from_value(tokenizer.to_owned()).unwrap();
    return Box::new(masking_tokenizer::BaseTokenizer::new(&config));
}
 
// Create the Endpoint for Squad
fn create_endpoint(value:&Arc<serde_yaml::Value>) -> Box<dyn crate::endpoint::EndPoint<MaskedData> + Send> {
    let tokenizer = &value["tokenizer"]["config"];
    let config:MaskingConfig = serde_yaml::from_value(tokenizer.to_owned()).unwrap();
    let endpoint = Box::new(MaskingEndpoint::new(config));
    return endpoint;
}



pub async fn run(value:Arc<Value>) -> bool{

    let result = generic_runner::run_main(value, 
        generic_runner::Either::Left(Box::new(create_provider)), 
        Box::new(create_generator) , 
        Box::new(create_endpoint));
    result.await 
}



