
use tokio::task::{JoinHandle, self};

use crate::{provider::{ProviderChannel, general_file_provider, pile_datasets, source_filter::SourceFilter, provider_config::{ProviderConfig, SourceDescription}}, tasks::{runner_simple}, datasets::DataSet, tokenizer::tokenizer_wrapper::TokenizerWrapper, config::{TrainingConfig, TaskType}, batcher::BatchConfig};
use tokio::sync::mpsc::Sender;

use super::{ gpt2_test_endpoint::Gpt2Endpoint, masking_test_endpoint::MaskingEndpoint,t5_test_endpoint::T5Endpoint};
use crate::tasks::gen_tokenizer::GenTokenizer;




// Create the Dataset Provider for Squad
pub fn create_provider(provider_config:ProviderConfig, tx:Sender<ProviderChannel<String>>, cache:Option<String>) -> JoinHandle<()> {


    //let provider_config:ProviderConfig = serde_yaml::from_value(value["source"].to_owned()).unwrap();
    let filter = provider_config.filter.unwrap_or(SourceFilter::JsonText);
    
    let handle = task::spawn(
        async move {
            match provider_config.source {
                SourceDescription::DataList(datasets) => {
                    //log::info!("Datasets {:?}", datasets);
                    general_file_provider::load_data_sets(datasets, provider_config.length, tx, cache, &filter).await;
                },
                SourceDescription::Pile{typ} => {
                    let datasets = pile_datasets::get_datasets(typ);
                    match datasets {
                        Some(x) => {
                            general_file_provider::load_data_sets(x, provider_config.length, tx, cache, &filter).await;
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




fn create_generator(batch_config:BatchConfig, dataset:DataSet, tokenizer:TokenizerWrapper)-> Box<dyn crate::batcher::Batcher<S=String,T=DataSet> + Send> {
    
    let wrap = GenTokenizer::new(dataset, batch_config, tokenizer, true);
    Box::new(wrap)
}

fn create_causal_generator(batch_config:BatchConfig, dataset:DataSet, tokenizer:TokenizerWrapper)-> Box<dyn crate::batcher::Batcher<S=String,T=DataSet> + Send> {
    
    let wrap = GenTokenizer::new(dataset, batch_config, tokenizer, true);
    Box::new(wrap)
}
 
fn create_t5_generator(batch_config:BatchConfig, dataset:DataSet, tokenizer:TokenizerWrapper)-> Box<dyn crate::batcher::Batcher<S=String,T=DataSet> + Send> {

    let wrap = GenTokenizer::new(dataset, batch_config, tokenizer, false);
    Box::new(wrap)
}


fn create_causal_endpoint(config:TrainingConfig) -> Box<dyn crate::transport::test_endpoint::EndPoint<DataSet> + Send> { 
    Box::new(Gpt2Endpoint::new(config))
}
pub fn create_endpoint(config:TrainingConfig) -> Box<dyn crate::transport::test_endpoint::EndPoint<DataSet> + Send> {
    Box::new(MaskingEndpoint::new(config))
}
fn create_t5_endpoint(config:TrainingConfig) -> Box<dyn crate::transport::test_endpoint::EndPoint<DataSet> + Send> {
    Box::new(T5Endpoint::new(config))
}

pub enum MaskType {
    Mlm,
    Causal, 
    Span
}

pub async fn run(config:TrainingConfig, cache:Option<String>) -> bool{

    let dataset = config.dataset.clone();
    match config.model.clone() {
        TaskType::Mlm => {
            runner_simple::run_main(config,
                dataset, 
                runner_simple::ProviderType::Sync(Box::new(create_provider)), 
                Box::new(create_generator), 
                Box::new(create_endpoint),
                cache
            ).await
        },
        TaskType::Causal => {
            runner_simple::run_main(config,
                dataset, 
                runner_simple::ProviderType::Sync(Box::new(create_provider)), 
                Box::new(create_causal_generator) , 
                Box::new(create_causal_endpoint),
                cache
            ).await
        },
        TaskType::T5 => {
            runner_simple::run_main(config,
                dataset, 
                runner_simple::ProviderType::Sync(Box::new(create_provider)), 
                Box::new(create_t5_generator) , 
                Box::new(create_t5_endpoint),
                cache
            ).await 
        },
        _ => {
            log::error!("Model not Support");
            false
        }
    }
    
}



