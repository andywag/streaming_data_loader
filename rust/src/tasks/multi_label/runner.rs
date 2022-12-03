

use crate::{provider::{arrow_transfer::ArrowTransfer, arrow_provider::{create_hugging_description}, provider_config::{SourceDescription, ProviderConfig}}, tasks::{multi_label::multi_arrow::MultiArrowGenerator, runner_simple}, tokenizer::tokenizer_wrapper::{TokenizerWrapper}, config::TrainingConfig, batcher::BatchConfig, datasets::DataSet};

use super::{multi_data::{MultiTransport, MultiData}, multi_tokenizer::{self}};

// Create the Dataset Provider for Squad
fn create_provider(config:&ProviderConfig) -> ArrowTransfer<MultiTransport>{
    match &config.source {
        SourceDescription::HuggingFace(x) => {
            let arrow_description = create_hugging_description(x.dataset.clone(), x.args.clone(), x.operations[0].clone());
            let mut loader = ArrowTransfer::new(arrow_description.0, arrow_description.1);
            let generator = Box::new(MultiArrowGenerator::new(&loader.schema)) ;
    
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

// Create the Tokenizer for Multi Label Data
fn create_generator(_batch_config:BatchConfig, dataset:DataSet, tokenizer:TokenizerWrapper)-> Box<dyn crate::batcher::Batcher<S=MultiTransport,T=MultiData> + Send> {
    match dataset {
        DataSet::Multi(x) => Box::new(multi_tokenizer::MultiTokenizer::new(x, tokenizer)),
        _ => {
            log::error!("Require Multi Data Input");
            std::process::exit(1);
        }
    }
}


// Top level run function
pub async fn run(config:TrainingConfig) -> bool{
    let dataset = config.dataset.clone();
    let result = runner_simple::run_main(
        config,
        dataset, 
        runner_simple::ProviderType::Async(Box::new(create_provider)), 
        Box::new(create_generator) , 
        Box::new(crate::transport::test_endpoint::default_endpoint),
    None);
    result.await 
}

