use std::sync::Arc;

use serde_yaml::Value;

use crate::{provider::{arrow_transfer::ArrowTransfer, arrow_provider}, datasets::{generic_runner, multi_label::multi_arrow::MultiArrowGenerator}};

use super::{multi_data::{MultiTransport, MultiData}, MultiConfig, multi_tokenizer::{self}, multi_test_endpoint::MultiTestEndpoint};


// Create the Dataset Provider for Squad
fn create_provider(_value:&Arc<serde_yaml::Value>) -> ArrowTransfer<MultiTransport>{
    let arrow_files = arrow_provider::download_huggingface_dataset("xed_en_fi".to_string(), Some("en_annotated".to_string())).unwrap();
    let arrow_train = arrow_files.get_locations("train".to_string()).unwrap();
    let arrow_location = arrow_train.0[0].to_owned();
    let arrow_length = arrow_train.1;

    let mut loader = ArrowTransfer::new(arrow_location, arrow_length);
    
    let generator = Box::new(MultiArrowGenerator::new(&loader.schema)) ;
    
    loader.generator = Some(generator);
    return loader;
}

// Create the Tokenizer for Squad
fn create_generator(value:&Arc<serde_yaml::Value>)-> Box<dyn crate::batcher::Batcher<S=MultiTransport,T=MultiData> + Send> {
    let tokenizer = &value["tokenizer"]["config"];
    let config:MultiConfig = serde_yaml::from_value(tokenizer.to_owned()).unwrap();
    return Box::new(multi_tokenizer::MultiTokenizer::new(&config));
}

// Create the Endpoint for Squad
fn create_endpoint(value:&Arc<serde_yaml::Value>) -> Box<dyn crate::endpoint::EndPoint<MultiData> + Send> {
    let tokenizer = &value["tokenizer"]["config"];
    let config:MultiConfig = serde_yaml::from_value(tokenizer.to_owned()).unwrap();
    let endpoint = Box::new(MultiTestEndpoint::new(config));
    return endpoint;
}

// TODO : The squad implementation has quite a few flaws and is not fully functional

pub async fn run(value:Arc<Value>) -> bool{

    let result = generic_runner::run_main(value, 
        generic_runner::Either::Right(Box::new(create_provider)), 
        Box::new(create_generator) , 
        Box::new(create_endpoint));
    result.await 
}

