
use logos::{Lexer, Logos};
use crate::tasks::python::python_line::create_import_context;
use crate::tasks::python::python_tokenizer::check_python;
use crate::tokenizer::tokenizer_data::TokenizedData;

use super::python_tokenizer::Token;
use super::context_store::{ContextStore};
use super::python_line::Line;



/// List of Tokens to ignore on a new line
fn ignore(token:&Token) -> bool {
    match token {
        Token::Comment | Token::AString | Token::TString | Token::String | Token::WS | Token::Newline | Token::Tab => true ,
        _ => false
    }
}

/// Splits the tokens into a set of lines
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
                line.push_number(token, lexer.slice().to_string());
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
}

 
// Struct which contains information about the python hierarchy derived from indents
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

    pub fn create_mask(&self, context:&ContextStore, s_length:&Vec<usize>, offset:u32, level:usize) -> (Vec<u32>, u32) {
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
 

/// Parse the lines in the file to create the context of the file
fn parse_lines(lines:&mut Vec<Line>, split:&mut Split, level:Option<usize>, line:usize, context:&mut ContextStore)  {
    
    let mut index = line;
    let mut current_child = Split::new(Holder::Body, split.sp);

    while index < lines.len() {
        if level.is_some() && lines[index].indent <= level.unwrap() { // Finish Condition on a Dedent
            if current_child.sp < index {
                current_child.ep = index;
                split.children.push(current_child);
            }
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
fn create_ids(lines:&mut Vec<Line>, context:&ContextStore) -> (Vec<u32>, Vec<u32>, Vec<u32>, Vec<usize>){
    let mut ids = Vec::<u32>::with_capacity(512);
    let mut positions = Vec::<u32>::with_capacity(512);
    let mut attentions = Vec::<u32>::with_capacity(512);
    let mut gaps = Vec::<usize>::with_capacity(128);

    let mut index = 0;
    for line in lines.as_mut_slice() {
        let data = line.create_ids(&context);
        ids.extend(data);
        let p:Vec<u32> = (0..line.length as u32).collect();
        positions.extend(p);
        attentions.extend(vec![index;line.length]);
        gaps.push(line.length);
        index += 1;
    }
    (ids, positions, attentions, gaps)
}



#[derive(Debug)]
pub struct PythonParserNew {
    context_size:usize,
}

impl PythonParserNew {
    pub fn new(context_size:usize) -> Self {
        Self {
            context_size:context_size-1, // Decremented One to Exclue Line Postions
        }
    }

    pub fn encode(&self, data:String) -> Option<TokenizedData> {

        if !check_python(&data) {
            return None;
        }
        
        let mut lexer = Token::lexer(data.as_str());
        // Convert the input file to a list of lines
        let base_lines = split_lines(&mut lexer).into_iter().collect();
        let import_context = create_import_context(&base_lines, 299);
        //log::info!("Import Context {:?}", import_context);

        let mut context = ContextStore::new(import_context, 300, 300);

        // Remove the import lines
        let mut lines = base_lines.into_iter().filter(|s|s.valid_line()).collect();
        //log::info!("Lines {:?}", lines);
        // Top Level Grouping for File
        let mut body = Split::new(Holder::Body, 0);
        // Parse the file
        parse_lines(&mut lines, &mut body, None, 0, &mut context);
        //log::info!("Body {:?}", body);
        // Create the ids and positions
        let (ids, pos, attn, gaps) = create_ids(&mut lines, &context);
        // Create a Line to Number of Token Index
        let s_length = create_lengths(&mut lines);
        // Create the Attention Indices
        let attention_size = self.context_size;
        let mut attn_ids = Vec::<Vec<u32>>::with_capacity(attention_size);
        for x in 0..self.context_size {
            let attn = body.create_mask(&context, &s_length, 0, x);
            attn_ids.insert(0,attn.0);
        }
        attn_ids.insert(0, attn);
        if attn_ids[0].len() != pos.len() {
            log::info!("Parser Failed {} {}", attn_ids.len(), pos.len());
            use std::fs;
            let _ = fs::write(format!("temp{}.py",1), data.clone());
        }


        // Update the Positions to the Smallest Group Size
        //log::info!("Size {} {} {} {}", id_position.1.len(), attn_ids[0].len(), attn_ids[1].len(), attn_ids[3].len());
        //log::info!("Size {:?} {:?}", id_position, attn_ids[0]);

        //let positions = PythonParserNew::create_positions(id_position.1, &attn_ids[0]); 
        
        
        Some(TokenizedData{ ids: ids, positions: pos, attention_mask: attn_ids, gaps: gaps }) 
    }
}
 

#[test]
pub fn test_full() {
    use std::fs::File;
    use std::io::Read;

    crate::logger::create_logger();
    let parser = PythonParserNew::new(4);

    let mut file = File::open("../python/temp.py").unwrap();
    let mut contents = String::new();
    let _= file.read_to_string(&mut contents);
    //log::info!("Data {}", contents);

    let result = parser.encode(contents);
    log::info!("R {:?}", result);

}

