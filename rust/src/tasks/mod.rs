use std::sync::Arc;

use serde::{Serialize,Deserialize};

pub mod masking;
pub mod multi_label;
pub mod squad;
pub mod single_class;

pub mod runner_simple;
pub mod gen_tokenizer;

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

pub async fn run(task_str:Option<&str>, config_ptr:Arc<serde_yaml::Value>) -> bool{
    match task_str {
        Some("squad") => squad::runner::run(config_ptr).await,//squad::runner::run(config_ptr, operations[0].to_owned()).await,
        Some("multi-label") => multi_label::runner::run(config_ptr).await,
        Some("single-class") => single_class::runner::run(config_ptr).await,
        Some("masking") => masking::masking_runner::run(config_ptr, false).await,
        Some("causal") => masking::masking_runner::run(config_ptr, true).await,
        Some(x) => {log::error!("Model {x} Not Found"); false}
        None => {log::error!("Model Not Found"); false}

    } 
    
}