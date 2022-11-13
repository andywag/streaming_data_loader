use std::sync::Arc;

use serde_yaml::Value;
use tokio::task::{JoinHandle, self};

use crate::{provider::{ProviderChannel, ProviderConfig, general_file_provider,  SourceDescription, pile_datasets}, tasks::{runner_simple}};
use tokio::sync::mpsc::Sender;

use super::{masking_tokenizer, MaskingConfig, masked_data::MaskedData, masking_test_endpoint::MaskingEndpoint, gpt2_tokenizer};





// Create the Dataset Provider for Squad
fn create_provider(value:&Arc<Value>, tx:Sender<ProviderChannel<String>>) -> JoinHandle<()> {


    let provider_config:ProviderConfig = serde_yaml::from_value(value["source"].to_owned()).unwrap();
    let handle = task::spawn(
        async move {
            match provider_config.source {
                SourceDescription::DataList(datasets) => {
                    //log::info!("Datasets {:?}", datasets);
                    general_file_provider::load_data_sets(datasets, provider_config.length, tx).await;
                },
                SourceDescription::Pile{typ} => {
                    let datasets = pile_datasets::get_datasets(typ);
                    match datasets {
                        Some(x) => {
                            general_file_provider::load_data_sets(x, provider_config.length, tx).await;
                        }
                        None => {
                            log::error!("Data Set Not Supported");
                            std::process::exit(0);
                        }
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

// Create the Tokenizer for Squad
fn create_causal_generator(value:&Arc<serde_yaml::Value>)-> Box<dyn crate::batcher::Batcher<S=String,T=MaskedData> + Send> {
    let tokenizer = &value["tokenizer"]["config"];
    let config:MaskingConfig = serde_yaml::from_value(tokenizer.to_owned()).unwrap();
    // TODO : Attach the premasked data for testing. Add Option to Select whether this is added
    return Box::new(gpt2_tokenizer::GPTTokenizer::new(&config, true));
}
 
// Create the Endpoint for Squad
fn create_endpoint(value:&Arc<serde_yaml::Value>) -> Box<dyn crate::transport::test_endpoint::EndPoint<MaskedData> + Send> {
    let tokenizer = &value["tokenizer"]["config"];
    let config:MaskingConfig = serde_yaml::from_value(tokenizer.to_owned()).unwrap();
    let endpoint = Box::new(MaskingEndpoint::new(config));
    return endpoint;
}



pub async fn run(value:Arc<Value>, causal:bool) -> bool{

    let generator = if causal {
        create_causal_generator
    }
    else {
        create_generator
    };
    let result = runner_simple::run_main(value, 
        runner_simple::Either::Left(Box::new(create_provider)), 
        Box::new(generator) , 
        Box::new(create_endpoint));
    result.await 
}



