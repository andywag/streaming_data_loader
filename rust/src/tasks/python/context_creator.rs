use counter::Counter;
use logos::Logos;

use crate::tokenizer::tokenizer_data::TokenizedData;

use super::python_tokenizer::Token;

//#[derive(Serialize, Deserialize)]
pub struct PythonContextCreator {
    counter:Counter<String>,
    path:String,
    size:usize,
    count:usize
}

impl PythonContextCreator {
    pub fn new(size:usize) -> Self{ 
        Self {
            counter:Counter::<String>::new(),
            path:"context.txt".to_string(),
            size:size,
            count:0
        }
    }

    pub fn write_contents(&self) {
        use std::fs::File;
        use std::io::Write;
        let mut f = File::create(&self.path).unwrap();
        let keys = self.counter.k_most_common_ordered(self.size);
        let key_string:Vec<String> = keys.into_iter().map(|s| format!("{} : {}", s.0, s.1)).collect();
        let result = key_string.join("\n");
        let _ = f.write_all(result.as_bytes());
        let _ = f.flush();
    }

    pub fn encode(&mut self, text:String) -> Option<TokenizedData> {
        let mut lexer = Token::lexer(&text);
        let mut idents = Vec::<String>::with_capacity(1024);
        while let Some(token) = lexer.next() {
            if token == Token::Ident {
                let slice = lexer.slice();
                let split = slice.split("_");
                for s in split {
                    if s.len() >= 2 {
                        idents.push(s.to_lowercase().to_string());            
                    }
                }
            }
        }
        self.counter += idents;
        self.count += 1;
        if self.count % 1024 == 1023 {
            self.write_contents();
            log::info!("Writing Contents");
        }
        
        None

    }
}