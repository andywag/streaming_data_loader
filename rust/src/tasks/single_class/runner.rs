
use crate::{provider::{arrow_transfer::ArrowTransfer, arrow_provider::{create_hugging_description}, provider_config::{SourceDescription, ProviderConfig}}, tasks::{ runner_simple}, tokenizer::tokenizer_wrapper::{TokenizerWrapper}, config::TrainingConfig, batcher::BatchConfig, datasets::DataSet};

use super::{single_data::{SingleClassTransport, SingleClassData}, single_arrow::SingleClassArrowGenerator};


// Create the Dataset Provider for Squad
fn create_provider(config:&ProviderConfig) -> ArrowTransfer<SingleClassTransport>{
    match &config.source {
        SourceDescription::HuggingFace(x) => {
            let arrow_description = create_hugging_description(x.dataset.clone(), x.args.clone(), x.operations[0].clone());
            let mut loader = ArrowTransfer::new(arrow_description.0, arrow_description.1);
            let generator = Box::new(SingleClassArrowGenerator::new(&loader.schema)) ;
    
            loader.generator = Some(generator);
            return loader;
        },
        SourceDescription::Arrow(_) => todo!(),
        _ => { 
            log::error!("Configuration Not Supported");
            std::process::exit(1);
        }
    };    
}

// Create the Tokenizer for Squad
fn create_generator(_batch_config:BatchConfig, dataset:DataSet,  tokenizer:TokenizerWrapper)-> Box<dyn crate::batcher::Batcher<S=SingleClassTransport,T=SingleClassData> + Send> {
    match dataset {
        DataSet::Single(x) => Box::new(super::tokenizer::SingleTokenizer::new(x, tokenizer)),
        _ => {
            log::error!("Require Multi Data Input");
            std::process::exit(1);
        }
    }
}


// TODO : The squad implementation has quite a few flaws and is not fully functional

pub async fn run(config:TrainingConfig) -> bool{
    let dataset = config.dataset.clone();
    let result = runner_simple::run_main(config,
        dataset, 
        runner_simple::ProviderType::Async(Box::new(create_provider)), 
        Box::new(create_generator), 
        Box::new(crate::transport::test_endpoint::default_endpoint),
        None);

    result.await 
}

