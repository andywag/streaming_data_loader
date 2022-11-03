use std::sync::Arc;

use serde::{Serialize,Deserialize};

pub mod masking;
pub mod multi_label;
pub mod squad;
pub mod single_class;

pub mod generic_runner;

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

pub async fn run(task:Task, config:Arc<serde_yaml::Value>) -> bool{
    match task {
        Task::Masking => masking::masking_runner::run(config).await,
        Task::SingleClass => single_class::runner::run(config).await,
        Task::MultiLabel => multi_label::runner::run(config).await,
        Task::Squad => squad::runner::run(config).await,
    }
}