
use logos::{Lexer, Logos};
use crate::tasks::python::context_store_new::ContextLookupNew;
use crate::tasks::python::python_tokenizer::check_python;
use crate::tokenizer::tokenizer_data::TokenizedData;

use super::config::PythonConfig;
use super::python_tokenizer::Token;
use super::context_store_new::ContextStoreNew;

#[derive(Debug)]
struct Ts {
    pub t:Token,
    pub d:Option<String>,
    pub id:Vec<(u32, u32)>
}

#[derive(Debug)]
enum Th {
    T(Token),
    I(Ts) 
}
impl Th {
    fn token(& self) -> Token{
        match self {
            Th::T(x) => x.to_owned(),
            Th::I(x) => x.t.to_owned(),
        }
    }
}


#[derive(Debug)]

struct Line {
    pub indent:usize,
    pub tokens:Vec<Th>,
    length:usize
}

impl Line {
    pub fn new(indent:usize) -> Self {
        Self { indent: indent, tokens:Vec::<Th>::with_capacity(32) , length:1}
    }
    pub fn push(&mut self, token:Token) {
        self.tokens.push(Th::T(token));
    }

    pub fn push_string(&mut self, token:Token, data:String) {
        self.tokens.push(Th::I(Ts{t: token, d: Some(data), id: vec![] }));
    }

    pub fn token(&mut self) -> Token {
        self.tokens[0].token()
    }

    pub fn update_context(&mut self, context:&mut ContextStoreNew) {
        for token in self.tokens.as_mut_slice() {
            match token {
                Th::I(x) => {
                    let text = x.d.as_mut().unwrap().as_str();
                    let g = text.split("_").filter(|s|s.len() >=1);
                    let ids = g.map(|s| context.get_or_put_local(s)).collect();
                    x.id = ids;
                }
                _ => {}
            };
        }
    }
    pub fn create_ids(&mut self, context:&ContextStoreNew) -> (Vec<u32>, Vec<u32>){
        let mut ids = Vec::<u32>::with_capacity(2*self.tokens.len());
        let mut positions = Vec::<u32>::with_capacity(2*self.tokens.len());

        let mut index = 0;
        for token in self.tokens.as_slice() {
            match token {
                Th::T(y) => {
                    ids.push(y.get_token_id());
                    positions.push(4*index);
                    
                },
                Th::I(x) => {
                    for i in x.id.as_slice().iter().enumerate() {
                        if i.0 <= 3 {
                            ids.push(context.get_id(i.1));
                            positions.push(4*index + i.0 as u32);
                        }
                    }
                },
            }
            
            index += 1;
        }
        ids.push(Token::Newline.get_token_id());
        positions.push(4*index);

        self.length = ids.len();
        (ids, positions)
    }

}


fn ignore(token:&Token) -> bool {
    match token {
        Token::Comment | Token::AString | Token::TString | Token::String | Token::WS | Token::Newline | Token::Tab => true,
        _ => false
    }
}

fn split_lines(lexer:&mut Lexer<Token>) -> Vec<Line> {
    let mut lines = Vec::<Line>::with_capacity(256);
    let mut line = Line::new(0);

    while let Some(token) = lexer.next() {
        if !ignore(&token) {line.push(token); break;}
    }
    
    while let Some(token) = lexer.next() {
        match token {
            Token::Newline => {
                let mut indent = None;
                while let Some(itoken) = lexer.next() {
                    if !ignore(&itoken) {
                        lines.push(line);
                        line = Line::new(indent.unwrap_or(0));
                        match itoken {
                            Token::Ident | Token::Number => line.push_string(itoken,lexer.slice().to_string()),
                            y => line.push(y)
                        }
                        
                        break;
                    }
                    indent = match itoken {
                        Token::WS | Token::Tab => Some(lexer.span().len()),
                        _ => None
                    };
                }
            },
            Token::Ident => {
                line.push_string(token, lexer.slice().to_string());
            }
            Token::Number => {
                line.push_string(token, lexer.slice().to_string());
            }
            Token::WS | Token::Tab => {

            }
            x => {
                line.push(x);
            }

        }
    }
    lines.push(line);
    lines

}

#[derive(Debug)]
enum Holder {
    Body,
    Def,
    Class,
    //Try
}

 
#[derive(Debug)]
struct Split {
    _typ:Holder,
    sp:usize,
    ep:usize,
    children:Vec<Split>
}


impl Split {
    pub fn new(_typ:Holder, sp:usize) -> Self {
        Self{ _typ, sp, ep:sp, children: Vec::with_capacity(4) }
    }

    pub fn create_mask(&self, context:&ContextStoreNew, s_length:&Vec<usize>, offset:u32, level:usize) -> (Vec<u32>, u32) {
        let l = s_length[self.ep] - s_length[self.sp];
 
        let mut result = Vec::with_capacity(level);
        let mut off = offset;

        if level == 0 || self.children.len() == 0 {
            result = vec![off;l];
            off += 1;
        }
        else {
            if level == 0 {    
                for child in self.children.as_slice() {
                    let ml = s_length[child.ep] - s_length[child.sp];
                    result.extend(vec![off;ml]);
                    off += 1;
                }
            }
            else {
                for child in self.children.as_slice() {
                    let res = child.create_mask(context, s_length, off, level-1);
                    result.extend(res.0);
                    off = res.1;
                }
            }
        }

        (result, off)
    }


}
 


fn parse_lines(lines:&mut Vec<Line>, split:&mut Split, level:Option<usize>, line:usize, context:&mut ContextStoreNew)  {
    
    let mut index = line;
    let mut current_child = Split::new(Holder::Body, split.sp);

    while index < lines.len() {
        if level.is_some() && lines[index].indent <= level.unwrap() { // Finish Condition on a Dedent
            split.ep = index;
            return;
        }
        let token = lines[index].token();
        match lines[index].token() {
            Token::KeyDef | Token::KeyClass => {
                if index != current_child.sp {  // If Valid Child (More than 1 Line) Add Child
                    current_child.ep = index;
                    split.children.push(current_child);
                }
                // Create New Child of Type Key/Class
                let holder_type = if token == Token::KeyDef {Holder::Def} else {Holder::Class};
                current_child = Split::new(holder_type, index);
                // Update Context and Parse Lines
                context.push_context();
                lines[index].update_context(context);
                parse_lines(lines, &mut current_child, Some(lines[index].indent), index+1, context);
                context.pop_context();
                // Update Index to Last Node
                index = current_child.ep; 
                // Add Current Child to the List of Nodes
                split.children.push(current_child);
                // Create New Current Child Starting on This Index
                current_child = Split::new(Holder::Body, index);
                               
            }   
            _ => {
                lines[index].update_context(context);
                index += 1;
            }
        }        
    }
    split.ep = index;
    if split.children.len() != 0 && current_child.sp != index{
        current_child.ep = index;
        split.children.push(current_child);
    }
    //split.ep = index;
}

/// Create a cumulative length table for the tokens per line
fn create_lengths(lines:&mut Vec<Line>) -> Vec<usize> {
    let mut s_length = Vec::<usize>::with_capacity(lines.len()+1);
    s_length.push(0);
    for x in 0..lines.len() {
        s_length.push(s_length[x] + lines[x].length);
    }
    s_length
}
/// Create a set of ids for each of the lines
fn create_ids(lines:&mut Vec<Line>, context:&ContextStoreNew) -> (Vec<u32>, Vec<u32>){
    let mut ids = Vec::<u32>::with_capacity(512);
    let mut positions = Vec::<u32>::with_capacity(512);

    for line in lines.as_mut_slice() {
        let data = line.create_ids(&context);
        ids.extend(data.0);
        positions.extend(data.1);
    }
    (ids,positions)
}

#[derive(Debug)]
pub struct PythonParserNew {
    global_store:ContextLookupNew,
    project_store:ContextLookupNew,
    config:PythonConfig,
    _index:u32
}

impl PythonParserNew {
    pub fn new(config:PythonConfig) -> Self {
        let project_store = ContextLookupNew::new(1024);

        Self {
            global_store:ContextLookupNew::from_file("../data/python_ident.txt"),
            project_store:project_store,
            config:config,
            _index:0
        }
    }

    fn create_positions(current_position:Vec<u32>, current_attention:&Vec<u32>) -> Vec<u32> {
        let mut positions = Vec::<u32>::with_capacity(current_position.len());
        
        let mut last_attention = 5000;
        let mut position_offset = 0;
        for x in 0..current_position.len() {
            if current_position[x] == 0 {
                if current_attention[x] != last_attention {
                    position_offset = 0;
                    last_attention = current_attention[x];
                }
                else {
                    position_offset += current_position[x-1] + 4;
                    position_offset -= current_position[x-1] % 4;
                }
            }
            positions.push(current_position[x] + position_offset);
        }
        positions
    }

    pub fn encode(&self, data:String) -> Option<TokenizedData> {

        if !check_python(&data) {
            return None;
        }
        //log::info!("Parser Failed");
        //use std::fs;
        //fs::write(format!("temp{}.py",1), data.clone());

        //log::info!("Project {:?}", data);
        // Create the Context which will be used to parse this file
        let mut context = ContextStoreNew::new(&self.global_store, 
            &self.project_store, 128, 1024);
        
        let mut lexer = Token::lexer(data.as_str());
        // Convert the input file to a list of lines
        let mut lines = split_lines(&mut lexer);
        //log::info!("Lines {:?}", lines);
        // Top Level Grouping for File
        let mut body = Split::new(Holder::Body, 0);
        // Parse the file
        parse_lines(&mut lines, &mut body, None, 0, &mut context);
        //log::info!("Body {:?}", body);
        // Create the ids and positions
        let id_position = create_ids(&mut lines, &context);
        // Create a Line to Number of Token Index
        let s_length = create_lengths(&mut lines);
        // Create the Attention Indices
        let attention_size = self.config.context_shape.len();
        let mut attn_ids = Vec::<Vec<u32>>::with_capacity(attention_size);
        for x in 0..self.config.context_shape.len() {
            let attn = body.create_mask(&context, &s_length, 0, x);
            attn_ids.insert(0,attn.0);
        }
        // Update the Positions to the Smallest Group Size
        //log::info!("Size {} {}", id_position.1.len(), attn_ids[attention_size-1].len());
        //log::info!("Size {:?} {:?}", id_position, attn_ids[attention_size-1]);

        let positions = PythonParserNew::create_positions(id_position.1, &attn_ids[0]); 
        
        
        Some(TokenizedData{ ids: id_position.0, positions: positions, attention_mask: attn_ids }) 
    }
}
/* 
#[test]
pub fn test_full() {
    use std::fs::File;
    use std::io::Read;

    crate::logger::create_logger();
    let config = PythonConfig{ mask_length: 32, context_shape: vec![2,2,4,4] };
    let parser = PythonParserNew::new(config);

    let mut file = File::open("../python/temp2.py").unwrap();
    let mut contents = String::new();
    let _= file.read_to_string(&mut contents);
    log::info!("Data {}", contents);

    let result = parser.encode(contents);
    log::info!("R {:?}", result);

}


///
#[test]
pub fn test_file() {
    use std::fs::File;
    use std::io::Read;

    crate::logger::create_logger();
    
    let mut global_store= ContextLookupNew::from_file("../data/python_ident.txt");
    let mut project_store = ContextLookupNew::new(1024);

    let mut file = File::open("../python/temp2.py").unwrap();
    let mut contents = String::new();
    let _= file.read_to_string(&mut contents);
   
    let mut lexer = Token::lexer(contents.as_str());
    let mut lines = split_lines(&mut lexer);

    let mut context_store = ContextStoreNew::new(&mut global_store, &mut project_store, 128, 1024);
    let mut body = Split::new(Holder::Body, 0);

     


    parse_lines(&mut lines, &mut body, None, 0, &mut context_store);

    //let _tokens = lex_simple(contents.as_str(), &mut global_store);
    for line in lines.as_mut_slice() {
        println!("Line: {:?}", line);
        println!("Ids: {:?}\n", line.create_ids(&context_store));
    }
    let s_length = create_lengths(&mut lines);

    
    println!("Body {:?}", body);
    println!("Slength {:?}", s_length);
    for x in 0..4 {
        let attn = body.create_mask(&context_store, &s_length, 0, x);
        println!("Attention {:?}", attn);
    }

    
}

*/
