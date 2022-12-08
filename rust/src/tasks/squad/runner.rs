

use crate::{provider::{arrow_transfer::ArrowTransfer, arrow_provider, provider_config::ProviderConfig}, tasks::{runner_simple}, tokenizer::tokenizer_wrapper::{TokenizerWrapper}, config::TrainingConfig, batcher::BatchConfig, datasets::{dataset::DataSet, dataset_config::DataSetConfig}};

use super::{squad_data::{SquadGeneral, SquadData}, squad_arrow::SquadArrowGenerator,squad_endpoint::SquadEnpoint};




// Create the Dataset Provider for Squad
fn create_provider(_config:&ProviderConfig) -> ArrowTransfer<SquadGeneral>{
    let arrow_files = arrow_provider::download_huggingface_dataset("squad".to_string(), None).unwrap();
    let arrow_train = arrow_files.get_locations("train".to_string()).unwrap();
    let arrow_location = arrow_train.0[0].to_owned();
    let arrow_length = arrow_train.1;

    //println!("Locations {:?}", locations);
    let mut loader = ArrowTransfer::new(arrow_location, arrow_length);
    let generator = Box::new(SquadArrowGenerator::new(&loader.schema)) ;
    loader.generator = Some(generator);
    return loader;
}

// Create the Tokenizer for Squad
fn create_generator(_batch_config:BatchConfig, dataset:DataSet, _config:DataSetConfig, tokenizer:TokenizerWrapper)-> Box<dyn crate::batcher::Batcher<S=SquadGeneral,T=SquadData> + Send> {
    match dataset {
        DataSet::Squad(x) => Box::new(super::squad_tokenizer::SquadTokenizer::new(x, tokenizer)),
        _ => {
            log::error!("Require Squad Input");
            std::process::exit(1);
        }
    }
}

// Create the Endpoint for Squad
fn create_endpoint(config:TrainingConfig) -> Box<dyn crate::transport::test_endpoint::EndPoint<SquadData> + Send> {
    //let tokenizer = &value["tokenizer"]["config"];
    //let config:SquadConfig = serde_yaml::from_value(tokenizer.to_owned()).unwrap();
    let endpoint = Box::new(SquadEnpoint::new(config));
    return endpoint;
}

// TODO : The squad implementation has quite a few flaws and is not fully functional

pub async fn run(config:TrainingConfig) -> bool{
    let dataset = config.dataset.clone();
    let result = runner_simple::run_main(
        config,
        dataset, 
        runner_simple::ProviderType::Async(Box::new(create_provider)), 
        Box::new(create_generator) , 
        Box::new(create_endpoint),
        None);
    result.await 
}

