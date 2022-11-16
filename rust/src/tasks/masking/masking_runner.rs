use std::sync::Arc;

use serde_yaml::Value;
use tokio::task::{JoinHandle, self};

use crate::{provider::{ProviderChannel, ProviderConfig, general_file_provider,  SourceDescription, pile_datasets}, tasks::{runner_simple}, tokenizer_wrapper::{self}, datasets::DataSet};
use tokio::sync::mpsc::Sender;

use super::{MaskingConfig, gpt2_test_endpoint::Gpt2Endpoint, gpt_data::GptData, masked_data::MaskedData, masking_test_endpoint::MaskingEndpoint};
use crate::tasks::gen_tokenizer::GenTokenizer;




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

fn get_config(value:&Arc<serde_yaml::Value>) -> MaskingConfig {
    let tokenizer = &value["tokenizer"]["config"];
    serde_yaml::from_value(tokenizer.to_owned()).unwrap()
}



fn create_generator(value:&Arc<serde_yaml::Value>)-> Box<dyn crate::batcher::Batcher<S=String,T=DataSet> + Send> {
    
    let config = get_config(&value);
    let tokenizer = tokenizer_wrapper::get_tokenizer(config.tokenizer_name.clone()).unwrap();
    let dataset = MaskedData::new(config.clone(),tokenizer.mask_token().unwrap());
    let wrap = GenTokenizer::new(value, crate::datasets::DataSet::Mask(dataset), config.batch_size as usize, config.sequence_length as usize, tokenizer);
    Box::new(wrap)
}

fn create_causal_generator(value:&Arc<serde_yaml::Value>)-> Box<dyn crate::batcher::Batcher<S=String,T=DataSet> + Send> {
    
    let config = get_config(&value);
    let dataset = GptData::new(config.batch_size as usize, config.sequence_length as usize);
    let tokenizer = tokenizer_wrapper::get_tokenizer(config.tokenizer_name).unwrap();
    let wrap = GenTokenizer::new(value, crate::datasets::DataSet::Gpt2(dataset), config.batch_size as usize, config.sequence_length as usize, tokenizer);
    Box::new(wrap)
}
 


fn create_causal_endpoint(value:&Arc<serde_yaml::Value>) -> Box<dyn crate::transport::test_endpoint::EndPoint<DataSet> + Send> { 
    Box::new(Gpt2Endpoint::new(get_config(value)))
}
fn create_endpoint(value:&Arc<serde_yaml::Value>) -> Box<dyn crate::transport::test_endpoint::EndPoint<DataSet> + Send> {
    Box::new(MaskingEndpoint::new(get_config(value)))
}


pub async fn run(value:Arc<Value>, causal:bool) -> bool{


    if causal {
        let result = runner_simple::run_main(value, 
            runner_simple::Either::Left(Box::new(create_provider)), 
            Box::new(create_causal_generator), 
            Box::new(create_causal_endpoint));
        result.await 
    }
    else {
        let result = runner_simple::run_main(value, 
            runner_simple::Either::Left(Box::new(create_provider)), 
            Box::new(create_generator) , 
            Box::new(create_endpoint));
        result.await 
    }


    
}



