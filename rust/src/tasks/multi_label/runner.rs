use std::sync::Arc;

use serde_yaml::Value;

use crate::{provider::{arrow_transfer::ArrowTransfer, arrow_provider::{create_hugging_description}, ProviderConfig}, tasks::{multi_label::multi_arrow::MultiArrowGenerator, runner_simple}, tokenizer::tokenizer_wrapper::{TokenizerWrapper}};

use super::{multi_data::{MultiTransport, MultiData}, MultiConfig, multi_tokenizer::{self}};

// Create the Dataset Provider for Squad
fn create_provider(config:&ProviderConfig) -> ArrowTransfer<MultiTransport>{
    match &config.source {
        crate::provider::SourceDescription::HuggingFace(x) => {
            let arrow_description = create_hugging_description(x.dataset.clone(), x.args.clone(), x.operations[0].clone());
            let mut loader = ArrowTransfer::new(arrow_description.0, arrow_description.1);
            let generator = Box::new(MultiArrowGenerator::new(&loader.schema)) ;
    
            loader.generator = Some(generator);
            return loader;
        },
        crate::provider::SourceDescription::Arrow(_) => todo!(),
        _ => { 
            log::error!("Configuration Not Supported");
            std::process::exit(1);
        }
    };

}

// Create the Tokenizer for Squad
fn create_generator(value:&Arc<serde_yaml::Value>, tokenizer:TokenizerWrapper)-> Box<dyn crate::batcher::Batcher<S=MultiTransport,T=MultiData> + Send> {
    let config:MultiConfig = serde_yaml::from_value(value["tokenizer"]["config"].to_owned()).unwrap();
    let data = MultiData::new(&config);
    return Box::new(multi_tokenizer::MultiTokenizer::new(data, tokenizer));
}


// Top level run function
pub async fn run(value:Arc<Value>) -> bool{

    let result = runner_simple::run_main(value, 
        runner_simple::Either::Right(Box::new(create_provider)), 
        Box::new(create_generator) , 
        Box::new(crate::transport::test_endpoint::default_endpoint),
    None);
    result.await 
}

