
use serde::{Deserialize, Serialize};



#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum DataSetConfig {
    Mask{mask_length:usize, mask:u32},
    Gpt,
    Span{avg_span_gap:f64, avg_span_size:f64, },
    MultiLabel{number_labels:usize},
    Squad,
    SingleClass,
    MaskHier{mask_length:usize, context_size:usize, front:bool},
    SpanHier{avg_span_prob:f64, context_size:usize},

}

