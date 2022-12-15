
use std::sync::mpsc::SyncSender;

use tokio::task::{JoinHandle, self};

use crate::{provider::{ProviderChannel, general_file_provider, pile_datasets, source_filter::SourceFilter, provider_config::{ProviderConfig, SourceDescription}}, tasks::{runner_simple}, datasets::{dataset::DataSet}, tokenizer::tokenizer_wrapper::{self}, config::{TrainingConfig}};
use tokio::sync::mpsc::Sender;

use super::{masking_test_endpoint::MaskingEndpoint};
use crate::tasks::gen_batcher::GenTokenizer;




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




fn create_generator(config:TrainingConfig)-> Box<dyn crate::batcher::Batcher<S=String,T=DataSet> + Send> {
    let wrap = GenTokenizer::new(config.model_config,  
        config.batch, 
        config.dataset_config,  
        tokenizer_wrapper::get_tokenizer(config.tokenizer).unwrap(), 
        true);
    Box::new(wrap)
}


pub fn create_endpoint(config:TrainingConfig) -> Box<dyn crate::transport::test_endpoint::EndPoint<DataSet> + Send> {
    Box::new(MaskingEndpoint::new(config))
}


pub enum MaskType {
    Mlm,
    Causal, 
    Span
}

pub async fn run(config:TrainingConfig, destination:Option<SyncSender<ProviderChannel<DataSet>>>, cache:Option<String>) -> bool{
    runner_simple::run_main(config,
        runner_simple::ProviderType::Sync(Box::new(create_provider)), 
        Box::new(create_generator), 
        destination,
        cache
    ).await

    
}



