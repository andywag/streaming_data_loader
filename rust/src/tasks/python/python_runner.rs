use crate::config::TaskType;
use crate::tasks::runner_simple;
use crate::{config::TrainingConfig, batcher::BatchConfig, datasets::DataSet, tokenizer::tokenizer_wrapper::TokenizerWrapper};
use crate::tasks::masking::masking_runner::{create_endpoint, create_provider};

use super::python_batcher::PythonBatch;
use super::python_top_new::PythonParserNew;


fn create_generator(batch_config:BatchConfig, dataset:DataSet, _tokenizer:TokenizerWrapper)-> Box<dyn crate::batcher::Batcher<S=String,T=DataSet> + Send> {
    
    let config = match dataset.clone() {
        DataSet::Python(x) => x.config,
        _ => {
            log::error!("Python Dataset Required");
            std::process::exit(1);
        }
    };
    let tokenizer = PythonParserNew::new(config);
    let batch = PythonBatch::new(dataset, batch_config, tokenizer);
    Box::new(batch)
}



pub enum MaskType {
    Mlm,
    Causal, 
    Span
}

pub async fn run(config:TrainingConfig, cache:Option<String>) -> bool{

    let dataset = config.dataset.clone();
    match config.model.clone() {
        TaskType::Python => {
            runner_simple::run_main(config,
                dataset, 
                runner_simple::ProviderType::Sync(Box::new(create_provider)), 
                Box::new(create_generator), 
                Box::new(create_endpoint),
                cache
            ).await
        },
        _ => {
            log::error!("Model not Support");
            false
        }
    }
    
}



