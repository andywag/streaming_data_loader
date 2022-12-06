

use counter::Counter;
use logos::Logos;

use serde::{Serialize, Deserialize};
use super::python_tokenizer::Token;
use super::context_map::{ContextLookup, ContextStore};
use super::python_parser::{TokenResult, StateMachine};

/// Simple Checker for Valid Python
pub fn check_python(text:&str) -> bool{
    let mut lexer = Token::lexer(text);
    let mut key_count = 0;
    while let Some(token) = lexer.next() {
        match token {
             Token::KeyDel | Token::KeyDef | Token::KeyElif | Token::KeyExcept => {key_count += 1;}
            _ => {}
        }
        if key_count > 8 {
            return true;
        }
    }
    return false;
}

/// Handle the python newlines to determine the level
pub fn get_level(lexer:&mut logos::Lexer<Token>, indent_width:&mut Option<usize>) -> (Option<Token>, usize) {
    let mut level = 0;
    while let Some(itoken) = lexer.next() {
        match itoken {
            Token::AString | Token::String | Token::TString => {}
            Token::Comment | Token::Newline=> {
                level = 0
            }
            Token::WS => {
                if indent_width.is_none() {
                    let ind = lexer.span().len();
                    *indent_width = Some(ind);
                }
                level = lexer.span().len()/indent_width.unwrap();
            }
            Token::Tab => {
                level = lexer.span().len()
            }
            x => {
                return (Some(x), level);
            }
        }
    }
    (None, 0)
}


fn lex_internal<'a>(text:&str, context:ContextStore<'a>) -> Option<Vec<TokenResult>> {
    let mut tokens = Vec::<TokenResult>::with_capacity(1024);

    let mut lexer = Token::lexer(text);

    let mut level = 0;
    let mut indent_width:Option<usize> = None;

    let mut state_machine = StateMachine::new(context);

    while let Some(token) = lexer.next() {
        match token {
            Token::WS | Token::Newline | Token::AString | Token::String | Token::TString | Token::Comment => {}
            _ => break
        };
    }
    while let Some(token) = lexer.next() {
        let (tok, lev) = match token {
            Token::WS | Token::Tab | Token::Comment => (None, level), // Ignore White space unless line beginning
            Token::Newline => {
                let result = get_level(&mut lexer, &mut indent_width);
                let token_result = TokenResult{ token:Token::Newline, level:0, position: None, text: None };
                tokens.push(token_result);
                //log::info!("Inside NewLine {:?}", result);
                let newline_ok = state_machine.newline(result.1);
                if newline_ok && result.1 <= level + 1 {
                    result
                }
                else {
                    (result.0, level)
                }
            } 
            _ => {
                (Some(token), level)
            }
        };
        level = lev;
        if tok.is_some() {
            let result = state_machine.put_token(tok.unwrap(), lexer.slice(), level);
            if result.is_none() {
                return None;
            }
            tokens.push(result.unwrap());
        }
    }
    if tokens.len() > 64 {
        return Some(tokens);
    }
    else {
        return None;
    }

}

/// Convert the python file to a list of tokens
pub fn lex_simple<'a>(text:&str, global_store:&'a mut ContextLookup) -> Option<Vec<TokenResult>> {
    if !check_python(text) {
        return None;
    }
    let local_store = &mut ContextLookup::new(1024);
    let context = ContextStore::new(global_store, local_store);
    let result = lex_internal(text, context);
    result

    
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PythonParserTop {
    global_store:ContextLookup,
    index:u32
}

impl PythonParserTop {
    pub fn new() -> Self {
        Self {
            global_store:ContextLookup::from_file("../data/python_ident.txt"),
            index:0
        }
    }
    pub fn encode(&mut self, data:String) -> Vec<u32> {
        //log::info!("Encoding Data {:?}", data);
        
        let mut ids = Vec::<u32>::with_capacity(512);
        let tokens_opt = lex_simple(&data, &mut self.global_store);
        if tokens_opt.is_none() {
            //log::info!("Parser Failed");
            //use std::fs;
            //fs::write(format!("temp{}.py",self.index), data.clone());
            self.index += 1;
            return Vec::<u32>::new();
        }
        let tokens = tokens_opt.unwrap();
        //log::info!("Slength {:?}", tokens.len());

        for token in tokens {
            //log::info!("Token {:?}", token);
            if token.token == Token::Ident {
                match token.position {
                    Some(p) => {
                        for x in p {
                            ids.push(x.0 as u32 + 10);
                            ids.push(x.1 as u32 + 200);
                        }
                        //ids.push(p[0].0 as u32 + 10);
                    },
                    None => {},
                }
            }
            else {
                ids.push(token.token.get_token_id());
            }
        }
        ids
    }


}

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

    pub fn encode(&mut self, text:String) -> Vec<u32> {
        let mut lexer = Token::lexer(&text);
        let mut idents = Vec::<String>::with_capacity(1024);
        while let Some(token) = lexer.next() {
            if token == Token::Ident {
                let slice = lexer.slice();
                let split = slice.split("_");
                for s in split {
                    if s.len() >= 3 {
                        idents.push(s.to_lowercase().to_string());            
                    }
                }
            }
        }
        self.counter += idents;
        self.count += 1;
        if self.count % 1024 == 1023 {
            self.write_contents();
        }
        
        vec![0;8]

    }
}


/* 
#[test]
pub fn test_python_token() {
    crate::create_logger();

    let mut global_store = IdentLookup::new(1024);
    //let context_store = ContextStore::new(&mut global_store);

    let _tokens = lex_simple("from c import d\nclass alpha:\n\tdef __init__(self):\n\t\tb = a + a + a\n", &mut global_store);
    //for token in tokens {
    //   println!("Token: {:?}", token);
    //}
    

}
*/
#[test]
pub fn test_file() {
    use std::fs::File;
    use std::io::Read;

    crate::logger::create_logger();
    let mut global_store= ContextLookup::from_file("../data/python_ident.txt");


    let mut file = File::open("../python/temp2.py").unwrap();
    let mut contents = String::new();
    let _= file.read_to_string(&mut contents);
   

    let _tokens = lex_simple(contents.as_str(), &mut global_store);
    //for token in _tokens {
    //    println!("Token: {:?}", token);
    //}
    
}


