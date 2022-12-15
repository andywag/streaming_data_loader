
use crate::{provider::{arrow_transfer::ArrowTransfer, arrow_provider::{create_hugging_description}, provider_config::{SourceDescription, ProviderConfig}}, tasks::{ runner_simple, multi_label::multi_arrow::MultiArrowGenerator}, tokenizer::tokenizer_wrapper::{self}, config::{TrainingConfig}, datasets::{dataset::DataSet, dataset_config::DataSetConfig}, models::{simple_batcher, simple_transport::SimpleTransport}};

use super::{single_arrow::SingleClassArrowGenerator};


// Create the Dataset Provider for Squad
fn create_provider(config:&ProviderConfig, data_config:DataSetConfig) -> ArrowTransfer<SimpleTransport>{
    match &config.source {
        SourceDescription::HuggingFace(x) => {
            let arrow_description = create_hugging_description(x.dataset.clone(), x.args.clone(), x.operations[0].clone());
            let mut loader = ArrowTransfer::new(arrow_description.0, arrow_description.1);
            match data_config {
                DataSetConfig::MultiLabel { number_labels:_ } => {
                    loader.generator = Some(Box::new(MultiArrowGenerator::new(&loader.schema)));
                },
                DataSetConfig::SingleClass => {
                    loader.generator = Some(Box::new(SingleClassArrowGenerator::new(&loader.schema)));
                },
                _ => {
                    log::error!("Configuration Not Supported"); 
                    std::process::exit(1);
                }
            };
            return loader
            //let generator = Box::new(SingleClassArrowGenerator::new(&loader.schema)) ;
    
            //loader.generator = Some(generator);
            //return loader;
        },
        SourceDescription::Arrow(_) => todo!(),
        _ => { 
            log::error!("Configuration Not Supported");
            std::process::exit(1);
        }
    };    
}

// Create the Batcher for Squad
fn create_generator(config:TrainingConfig)-> Box<dyn crate::batcher::Batcher<S=SimpleTransport,T=DataSet> + Send> {

    //let tokenizer = tokenizer_wrapper::get_tokenizer(config.tokenizer).unwrap();
    let batcher = simple_batcher::SimpleBatcher::new(config.model_config,
        config.dataset_config, 
        config.batch, 
        tokenizer_wrapper::get_tokenizer(config.tokenizer).unwrap());
    Box::new(batcher)
    
}


// TODO : The squad implementation has quite a few flaws and is not fully functional

pub async fn run(config:TrainingConfig) -> bool{
    let result = runner_simple::run_main(config,
        runner_simple::ProviderType::Async(Box::new(create_provider)), 
        Box::new(create_generator), 
        None,
        None);

    result.await 
}

