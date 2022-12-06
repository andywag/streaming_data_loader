use serde::{Deserialize, Serialize};



#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TokenizerConfig {
    #[serde(rename = "type")]
    pub typ: Option<String>,
    pub name: String,
    pub mode: Option<String>,
}

impl TokenizerConfig {
    pub fn to_internal(&self) -> TokenizerInternalConfig{
        let task =
            if self.name.contains("roberta") {TokenizerTask::Roberta}
            else if self.name.contains("bert") {TokenizerTask::Bert}
            else if self.name.contains("t5") {TokenizerTask::T5}
            else if self.name.contains("gpt") {TokenizerTask::Gpt}
            else {
                log::error!("Can't Find Tokenizer Task {:?}", self.name);
                std::process::exit(1);
            };
            
            let typ = match self.mode.clone() {
                Some(x) => {
                    if x == "python" {TokenizerType::Python} 
                    else {TokenizerType::HuggingFace(self.name.clone())}
                }
                None => TokenizerType::HuggingFace(self.name.clone())
            };
    
        TokenizerInternalConfig {
            task: task,
            typ: typ,
        }
    }
    
}

#[derive(Deserialize, Serialize, Debug, Clone)]

pub enum TokenizerTask {
    Bert,
    Roberta,
    T5,
    Gpt
}
#[derive(Deserialize, Serialize, Debug, Clone)]

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


pub enum Examples {
    Basic
}

impl Examples {
    pub fn get_config(&self, task:TokenizerTask, name:String) -> TokenizerInternalConfig {
        match self {
            Examples::Basic => TokenizerInternalConfig {
                task: task,
                typ: TokenizerType::HuggingFace(name),
            },
        }
    }
}