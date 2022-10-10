
#[derive(Clone)]
pub struct MaskingConfig{
    pub batch_size:u32,
    pub sequence_length:u32,
    pub mask_length:u32,
    pub tokenizer_name:String
}