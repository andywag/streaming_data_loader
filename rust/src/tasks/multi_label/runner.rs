use std::sync::Arc;

use serde_yaml::Value;

use crate::{provider::{arrow_transfer::ArrowTransfer, arrow_provider::{create_hugging_description}}, tasks::{generic_runner, multi_label::multi_arrow::MultiArrowGenerator}};

use super::{multi_data::{MultiTransport, MultiData}, MultiConfig, multi_tokenizer::{self}};

// Create the Dataset Provider for Squad
fn create_provider(_value:&Arc<serde_yaml::Value>) -> ArrowTransfer<MultiTransport>{
    let arrow_description = create_hugging_description("xed_en_fi".to_string(), Some("en_annotated".to_string()), "train".to_string());
    let mut loader = ArrowTransfer::new(arrow_description.0, arrow_description.1);
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


// Top level run function
pub async fn run(value:Arc<Value>) -> bool{

    let result = generic_runner::run_main(value, 
        generic_runner::Either::Right(Box::new(create_provider)), 
        Box::new(create_generator) , 
        Box::new(crate::endpoint::default_endpoint));
    result.await 
}

