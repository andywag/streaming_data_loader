use std::sync::Arc;

use serde_yaml::Value;

use crate::{provider::{arrow_transfer::ArrowTransfer, arrow_provider}, datasets::generic_runner};

use super::{squad_data::{SquadGeneral, SquadData}, squad_arrow_sync::SquadArrowGenerator, squad_tokenizer, SquadConfig, squad_endpoint::SquadEnpoint};


// Create the Dataset Provider for Squad
fn create_provider() -> ArrowTransfer<SquadGeneral>{
    let locations = arrow_provider::download_huggingface_dataset("squad".to_string(), None, "train".to_string());
    println!("Locations {:?}", locations);
    let mut loader = ArrowTransfer::new(locations.unwrap()[0].to_owned());
    let generator = Box::new(SquadArrowGenerator::new(&loader.schema)) ;
    loader.generator = Some(generator);
    return loader;
}

// Create the Tokenizer for Squad
fn create_generator(value:&Arc<serde_yaml::Value>)-> Box<dyn crate::batcher::Batcher<S=SquadGeneral,T=SquadData> + Send> {
    let tokenizer = &value["tokenizer"]["config"];
    let config:SquadConfig = serde_yaml::from_value(tokenizer.to_owned()).unwrap();
    return Box::new(squad_tokenizer::SquadTokenizer::new(&config));
}

// Create the Endpoint for Squad
fn create_endpoint(value:&Arc<serde_yaml::Value>) -> Box<dyn crate::endpoint::EndPoint<SquadData> + Send> {
    let tokenizer = &value["tokenizer"]["config"];
    let config:SquadConfig = serde_yaml::from_value(tokenizer.to_owned()).unwrap();
    let endpoint = Box::new(SquadEnpoint::new(config));
    return endpoint;
}

pub async fn run(value:Arc<Value>) -> bool{
    //let provider = SquadDataProvider{} as dyn ModelDataProvider<SquadGeneral> + Send;

    let result = generic_runner::run_main(value, 
        Box::new(create_provider), 
        Box::new(create_generator) , 
        Box::new(create_endpoint));
    result.await 
    //true
}