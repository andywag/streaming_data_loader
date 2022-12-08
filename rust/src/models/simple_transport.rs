use super::simple_label::Label;
#[derive(Clone, Debug)]

pub struct SimpleData {
    pub text:String,
    pub alt_text:Option<String>
}

impl From<(String,Option<String>)> for SimpleData {
    fn from(x: (String,Option<String>)) -> Self {
        Self { text: x.0, alt_text: x.1 }
    }
}

#[derive(Clone, Debug)]
pub struct SimpleTransport {
    pub data:SimpleData,
    pub label:Option<Label>
}