use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PythonConfig{
    pub mask_length:usize,
    pub context_shape:Vec<usize>,
}
