use serde::{Deserialize, Serialize};

/// Enum associated with the tokenizer operaton
#[derive(Deserialize, Serialize, Debug, Clone)]

pub enum TokenizerTask {
    Bert,
    Roberta,
    T5,
    Gpt
}
#[derive(Deserialize, Serialize, Debug, Clone)]

// Enum associated with the source of the tokenizer
pub enum TokenizerType {
    HuggingFace(String),
    Python,
    PythonContext
}


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TokenizerInternalConfig {
    pub task:TokenizerTask,
    pub typ:TokenizerType
}

