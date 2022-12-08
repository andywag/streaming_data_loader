use crate::config::TaskType;
use crate::datasets::dataset_config::DataSetConfig;
use crate::tasks::runner_simple;
use crate::tokenizer::tokenizer_data::TokenizedData;
use crate::{config::TrainingConfig, batcher::BatchConfig, datasets::dataset::DataSet, tokenizer::tokenizer_wrapper::TokenizerWrapper};
use crate::tasks::masking::masking_runner::{create_endpoint, create_provider};

use super::config::PythonConfig;
use super::context_creator::PythonContextCreator;
use super::python_batcher::PythonBatch;
use super::python_top_new::PythonParserNew;

pub enum PythonTokenizer {
    Run(PythonParserNew),
    Context(PythonContextCreator)
}

impl PythonTokenizer {
    pub fn encode(&mut self, data:String) -> Option<TokenizedData> {
        match self {
            PythonTokenizer::Run(x) => x.encode(data),
            PythonTokenizer::Context(x) => x.encode(data),
        }
    }
}

fn create_generator(batch_config:BatchConfig, _dataset:DataSet, dataset_config:DataSetConfig, _tokenizer:TokenizerWrapper)-> Box<dyn crate::batcher::Batcher<S=String,T=DataSet> + Send> {
    let config = match dataset_config.clone() {
        DataSetConfig::Python{mask_length,context_shape} => PythonConfig{mask_length, context_shape},
        _ => {
            log::error!("Python Dataset Required");
            std::process::exit(1);
        }
    };
    let tokenizer = PythonParserNew::new(config);
    let batch = PythonBatch::new( dataset_config, batch_config, PythonTokenizer::Run(tokenizer));
    Box::new(batch)
}

fn create_context_generator(batch_config:BatchConfig, _dataset:DataSet, dataset_config:DataSetConfig, _tokenizer:TokenizerWrapper)-> Box<dyn crate::batcher::Batcher<S=String,T=DataSet> + Send> {
    let tokenizer = PythonContextCreator::new(2048);
    let batch = PythonBatch::new( dataset_config, batch_config, PythonTokenizer::Context(tokenizer));
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
        TaskType::Context => {
            runner_simple::run_main(config,
                dataset, 
                runner_simple::ProviderType::Sync(Box::new(create_provider)), 
                Box::new(create_context_generator), 
                Box::new(create_endpoint),
                cache
            ).await
        }
        _ => {
            log::error!("Model not Support");
            false
        }
    }
    
}



