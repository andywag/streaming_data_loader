use crate::datasets::dataset_config::DataSetConfig;
use crate::tasks::runner_simple;
use crate::tokenizer::tokenizer_data::TokenizedData;
use crate::tokenizer::tokenizer_wrapper::TokenizerInfo;
use crate::{config::TrainingConfig, datasets::dataset::DataSet};
use crate::tasks::masking::masking_runner::{ create_provider};

use super::context_creator::PythonContextCreator;
use super::python_batcher::PythonBatch;
use super::python_parser::PythonParserNew;

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


    pub fn get_tokenizer_info(&self) -> TokenizerInfo {
        TokenizerInfo {
            cls: 1,
            sep: 1,
            pad: 0,
            mask: 5,
            unk: 4,
            extra: (2500..2600).collect(),
            eos: 2,
        }
    }

}

fn create_generator(config:TrainingConfig)-> Box<dyn crate::batcher::Batcher<S=String,T=DataSet> + Send> {
    let context_size = match config.dataset_config.clone() {
        DataSetConfig::MaskHier{mask_length:_,context_size, front:_} => context_size,
        DataSetConfig::SpanHier { avg_span_prob:_, context_size} => context_size,
        _ => {
            log::error!("Python Dataset Required");
            std::process::exit(1);
        }
    };
    let tokenizer = PythonParserNew::new(context_size);
    let batch = PythonBatch::new( config.model_config, config.dataset_config, config.batch, PythonTokenizer::Run(tokenizer));
    Box::new(batch)
}

fn create_context_generator(config:TrainingConfig)-> Box<dyn crate::batcher::Batcher<S=String,T=DataSet> + Send> {
    let tokenizer = PythonContextCreator::new(2048);
    let batch = PythonBatch::new( config.model_config, config.dataset_config, config.batch, PythonTokenizer::Context(tokenizer));
    Box::new(batch)
}


pub enum MaskType {
    Mlm,
    Causal, 
    Span
}

pub async fn run(config:TrainingConfig, cache:Option<String>) -> bool{

    runner_simple::run_main(config,
        runner_simple::ProviderType::Sync(Box::new(create_provider)), 
            Box::new(create_generator),
            None, 
            cache
        ).await
        
}


pub async fn run_context(config:TrainingConfig, cache:Option<String>) -> bool {

    runner_simple::run_main(config,
        runner_simple::ProviderType::Sync(Box::new(create_provider)), 
        Box::new(create_context_generator), 
        None,
        cache).await
            
}

