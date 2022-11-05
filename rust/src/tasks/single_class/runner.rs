use std::sync::Arc;

use serde_yaml::Value;

use crate::{provider::{arrow_transfer::ArrowTransfer, arrow_provider::{create_hugging_description}, ProviderConfig}, tasks::{ runner_simple}};

use super::{single_data::{SingleClassTransport, SingleClassData}, SingleClassConfig, single_arrow::SingleClassArrowGenerator};



// Create the Dataset Provider for Squad
fn create_provider(config:&ProviderConfig) -> ArrowTransfer<SingleClassTransport>{
    match &config.source {
        crate::provider::SourceDescription::HuggingFace(x) => {
            let arrow_description = create_hugging_description(x.dataset.clone(), x.args.clone(), x.operations[0].clone());
            let mut loader = ArrowTransfer::new(arrow_description.0, arrow_description.1);
            let generator = Box::new(SingleClassArrowGenerator::new(&loader.schema)) ;
    
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
fn create_generator(value:&Arc<serde_yaml::Value>)-> Box<dyn crate::batcher::Batcher<S=SingleClassTransport,T=SingleClassData> + Send> {
    let tokenizer = &value["tokenizer"]["config"];
    let config:SingleClassConfig = serde_yaml::from_value(tokenizer.to_owned()).unwrap();
    return Box::new(super::tokenizer::SingleTokenizer::new(&config));
}



// TODO : The squad implementation has quite a few flaws and is not fully functional

pub async fn run(value:Arc<Value>) -> bool{

    let result = runner_simple::run_main(value, 
        runner_simple::Either::Right(Box::new(create_provider)), 
        Box::new(create_generator), 
        Box::new(crate::test_endpoint::default_endpoint));

    result.await 
}

