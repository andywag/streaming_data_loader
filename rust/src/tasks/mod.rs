
use serde::{Serialize,Deserialize};

use crate::config::{TrainingConfig, TaskType};

use self::masking::masking_runner::MaskType;

pub mod masking;
pub mod multi_label;
pub mod squad;
pub mod single_class;

pub mod runner_simple;

pub mod gen_tokenizer;
pub mod python;
pub mod task_config;
pub mod cases;

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
        TaskType::Mlm => masking::masking_runner::run(config, cache, MaskType::Mlm).await,
        TaskType::Causal => masking::masking_runner::run(config,  cache, MaskType::Causal).await,
        TaskType::T5 => masking::masking_runner::run(config,  cache, MaskType::Span).await,

        //Some(x) => {log::error!("Model {x} Not Found"); false}
        //None => {log::error!("Model Not Found"); false}

    }    
}
