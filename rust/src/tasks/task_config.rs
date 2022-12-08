
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TaskConfig {
    Single,
    Multi,
    Squad
}

impl TaskConfig {
    
}