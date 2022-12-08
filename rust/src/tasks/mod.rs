
use serde::{Serialize,Deserialize};

use crate::config::{TrainingConfig, TaskType};


pub mod masking;
pub mod multi_label;
pub mod squad;
pub mod single_class;

pub mod runner_simple;

pub mod gen_batcher;
pub mod python;
pub mod task_config;
pub mod cases;
pub mod arrow_cases;
//pub mod simple_batcher;

#[derive(Debug, Serialize, Deserialize)]
pub struct DatasetInfo {
    pub name:String,
    pub length:u32
}

pub enum Task {
    Masking,
    SingleClass,
    MultiLabel,
    Squad
}

pub async fn run(config:TrainingConfig,  cache:Option<String>) -> bool{
    match config.model {
        TaskType::Squad => squad::runner::run(config).await,
        TaskType::MultiLabel => multi_label::runner::run(config).await,
        TaskType::SingleClass => single_class::runner::run(config).await,
        TaskType::Mlm => masking::masking_runner::run(config, cache).await,
        TaskType::Causal => masking::masking_runner::run(config,  cache).await,
        TaskType::T5 => masking::masking_runner::run(config,  cache).await,
        TaskType::Python => python::python_runner::run(config, cache).await,
        TaskType::Context => true
        
    }    
}
