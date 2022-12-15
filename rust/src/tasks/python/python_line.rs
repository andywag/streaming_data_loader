use super::{python_tokenizer::{Token, Operator}, context_store::{ContextStore, ContextSet}};

/// Structure which contains a token identifier
#[derive(Debug)]
pub struct Ts {
    pub t:Token,
    pub d:Option<String>,
    pub id:u32
}

#[derive(Debug)]
pub enum Th {
    T(Token),
    I(Ts),
    N(Ts) 
}
impl Th {
    fn token(& self) -> Token{
        match self {
            Th::T(x) => x.to_owned(),
            Th::I(x) => x.t.to_owned(),
            Th::N(x) => x.t.to_owned()
        }
    }
}


/// Python Line
/// Structure which holds the indent and list of tokens
#[derive(Debug)]

pub struct Line {
    pub indent:usize,
    pub tokens:Vec<Th>,
    pub length:usize
}

impl Line {
    pub fn new(indent:usize) -> Self {
        Self { indent: indent, tokens:Vec::<Th>::with_capacity(32) , length:1}
    }
    pub fn push(&mut self, token:Token) {
        self.tokens.push(Th::T(token));
    }

    pub fn push_string(&mut self, token:Token, data:String) {
        self.tokens.push(Th::I(Ts{t: token, d: Some(data), id: 0 }));
    }
    
    pub fn push_number(&mut self, token:Token, data:String) {
        let value = data.parse::<u32>();
        match value {
            Ok(x) => {
                if x <= 10 {
                    self.tokens.push(Th::N(Ts{t: token, d: Some(data), id: 148 + x + 1 })) 
                }
                else {
                    self.tokens.push(Th::N(Ts{t: token, d: Some(data), id: 148 }))
                }
            },
            Err(_) => self.tokens.push(Th::N(Ts{t: token, d: Some(data), id: 148 }))
        }   
    }


    pub fn token(&self) -> Token {
        self.tokens[0].token()
    }
    pub fn valid_line(&self) -> bool {
        self.tokens.len() > 0 && self.token() != Token::KeyImport && self.token() != Token::KeyFrom
    }

    fn handle_normal_line(&mut self, context:&mut ContextStore) {
        for token in self.tokens.as_mut_slice() {
            if let Th::I(x) = token {
                x.id = context.read_only(x.d.as_mut().unwrap().as_str());
            }
        }
    }
    fn handle_assign(&mut self, context:&mut ContextStore, mut level:usize) {
        for token in self.tokens.as_slice() {
            match token {
                Th::T(Token::SymbolLeftParen) => return self.handle_normal_line(context),
                Th::T(Token::Op(Operator::Assign)) => break,
                _ => {},
            }
        }
        let mut write = true;
        for token in self.tokens.as_mut_slice() {
            if let Th::I(x) = token {
                x.id = context.put_data(x.d.as_mut().unwrap().as_str(), write, level);
                write = false;
                level = 0;
            }
        }
    }
    fn handle_def(&mut self, context:&mut ContextStore) {
        let mut write = true;
        let mut level = 1;
        for token in self.tokens.as_mut_slice() {
            if let Th::I(x) = token {
                x.id = context.put_data(x.d.as_mut().unwrap().as_str(), write, level);
                write = true;
                level = 0;
            }
            if let Th::T(Token::SymbolColon) = token {
                write = false;
            }
        }
    }
    fn handle_for(&mut self, context:&mut ContextStore) {
        let mut write = true;
        let mut level = 1;
        for token in self.tokens.as_mut_slice() {
            if let Th::I(x) = token {
                x.id = context.put_data(x.d.as_mut().unwrap().as_str(), write, level);
                write = true;
                level = 0;
            }
            if let Th::T(Token::KeyIn) = token {
                write = false;
            }
        }
    }



    pub fn update_context(&mut self, context:&mut ContextStore) {
        match self.token() {
            Token::Ident => self.handle_assign(context, 0),
            Token::KeyDef => self.handle_def(context),
            Token::KeyClass => self.handle_def(context),
            Token::KeyFor => self.handle_for(context),
            _ => self.handle_normal_line(context)
        }

        
    }
    pub fn create_ids(&mut self, _context:&ContextStore) -> Vec<u32>{
        let mut ids = Vec::<u32>::with_capacity(2*self.tokens.len());

        for token in self.tokens.as_slice() {
            match token {
                Th::T(y) => {
                    ids.push(y.get_token_id());                    
                },
                Th::I(x) => {
                    ids.push(x.id);
                    //ids.push(147)
                },
                Th::N(x) => {
                    ids.push(x.id);
                    //ids.push(147)
                },
            }
                    }
        ids.push(Token::Newline.get_token_id());

        self.length = ids.len();
        ids
    }

}

/// Convenience Class which Creates the Import Context
/// Context is just a list of possible tokens imported
/// TODO : Currently contains all identifiers on line should be pruned
pub fn create_import_context(lines:&Vec<Line>, id:u32) -> ContextSet {
    let mut context_map = ContextSet::new(id);
    for line in lines {
        if line.valid_line() {continue};
        for token in line.tokens.as_slice() {
            if let Th::I(x) = token {
                let value = x.d.to_owned().unwrap();
                context_map.items.insert(value);
            }
        }
    }
    context_map
}