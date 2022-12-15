
use std::sync::mpsc::SyncSender;

use serde::{Serialize,Deserialize};

use crate::{config::{TrainingConfig, TaskType}, datasets::dataset::DataSet, provider::ProviderChannel};


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


pub async fn run(config:TrainingConfig, task:TaskType, cache:Option<String>, destination:Option<SyncSender<ProviderChannel<DataSet>>>) -> bool{
    match task {
        TaskType::Squad => single_class::runner::run(config).await,
        TaskType::MultiLabel => single_class::runner::run(config).await,
        TaskType::SingleClass => single_class::runner::run(config).await,
        TaskType::Mlm => masking::masking_runner::run(config, destination, cache).await,
        TaskType::Clm => masking::masking_runner::run(config,  destination, cache).await,
        TaskType::Span => masking::masking_runner::run(config,  destination, cache).await,
        TaskType::Python => python::python_runner::run(config, cache).await,
        TaskType::SpanPython => python::python_runner::run(config, cache).await,

        TaskType::Context => true 
    } 
    
}
