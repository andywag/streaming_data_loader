


pub struct BatchDescription {
    pub batch_size:usize,
    pub sequence_length:usize,
    pub tokenizer_name:String
}

pub trait DataGenerator {
    type Label;
    fn new_data(&self) -> Self;
    //fn put_data(&mut self, data:Self::T) -> bool;
    fn put_data(&mut self, encoding:&tokenizers::Encoding, labels:&Self::Label) -> bool;
}

pub trait TextSupplier {
    type Label;
    fn text(&self) -> tokenizers::EncodeInput;
    fn labels(&self) -> Self::Label; 
}

