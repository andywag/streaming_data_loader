use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum SourceFilter {
    #[serde(rename = "json_text")]
    JsonText,
    #[serde(rename = "fast_text")]
    FastText,
    #[serde(rename = "python_text")]
    PythonText 
}

impl SourceFilter {
    pub fn get_text(&self, line:String) -> Option<String> {
        match self {
            SourceFilter::JsonText => super::provider_util::create_json_text(line, "text"),
            SourceFilter::PythonText => super::provider_util::create_json_python_text(line, "text"),
            SourceFilter::FastText => None,
        }
        
    }
}