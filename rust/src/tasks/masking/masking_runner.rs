use std::sync::Arc;

use serde_yaml::Value;
use tokio::task::{JoinHandle, self};

use crate::{provider::{ProviderChannel, wiki_file_provider, ProviderConfig}, tasks::{runner_simple}};
use tokio::sync::mpsc::Sender;

use super::{masking_tokenizer, MaskingConfig, masked_data::MaskedData, masking_test_endpoint::MaskingEndpoint};




// Create the Dataset Provider for Squad
fn create_provider(value:&Arc<Value>, tx:Sender<ProviderChannel<String>>) -> JoinHandle<()> {
    let provider_config:ProviderConfig = serde_yaml::from_value(value["source"].to_owned()).unwrap();
    let handle = task::spawn(
        async move {
            match provider_config.source {
                crate::provider::SourceDescription::Wiki(x) => {
                    if x.network { // URL to web version of file
                        wiki_file_provider::load_url(&x.location, provider_config.length, tx).await
                    }
                    else {// Downloaded File
                        wiki_file_provider::load_data(&x.location, provider_config.length, tx).await
                    }
                },
                _ => {
                    log::error!("Can't support Input Type");
                }
            }
        });
    handle

}

 
// Create the Tokenizer for Squad
fn create_generator(value:&Arc<serde_yaml::Value>)-> Box<dyn crate::batcher::Batcher<S=String,T=MaskedData> + Send> {
    let tokenizer = &value["tokenizer"]["config"];
    let config:MaskingConfig = serde_yaml::from_value(tokenizer.to_owned()).unwrap();
    // TODO : Attach the premasked data for testing. Add Option to Select whether this is added
    return Box::new(masking_tokenizer::BaseTokenizer::new(&config, true));
}
 
// Create the Endpoint for Squad
fn create_endpoint(value:&Arc<serde_yaml::Value>) -> Box<dyn crate::transport::test_endpoint::EndPoint<MaskedData> + Send> {
    let tokenizer = &value["tokenizer"]["config"];
    let config:MaskingConfig = serde_yaml::from_value(tokenizer.to_owned()).unwrap();
    let endpoint = Box::new(MaskingEndpoint::new(config));
    return endpoint;
}



pub async fn run(value:Arc<Value>) -> bool{

    let result = runner_simple::run_main(value, 
        runner_simple::Either::Left(Box::new(create_provider)), 
        Box::new(create_generator) , 
        Box::new(create_endpoint));
    result.await 
}



