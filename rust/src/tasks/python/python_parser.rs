use super::{python_tokenizer::Token, context_map::{ ContextStore}};

#[derive(Debug)]
pub struct TokenResult {
    pub token:Token,
    pub level:usize,
    pub position:Option<Vec<(u32,u32)>>,
    pub text:Option<String>
}


#[derive(Clone, Debug)]
enum IdentState {
    Write,
    Read,
    Local,
}

#[derive(Clone, Debug)]
enum State {
    Root,
    Body,
    CHead,
    DHead,
    DParam,
    
    
}

pub struct StateMachine<'a> {
   state:Vec<State>,
   context:ContextStore<'a>,
   level:usize,
   ident_state:IdentState,
   ready_line:bool
   
}

impl <'a>StateMachine<'a> {

    pub fn new(context:ContextStore<'a>) -> Self {
        Self {
            state:vec!(State::Root),
            context:context,
            level:0,
            ident_state:IdentState::Read,
            ready_line:false
        }
    }

    fn replace_head(&mut self, state:State) {
        let l = self.state.len() - 1;
        self.state[l] = state;
    }


    pub fn newline(&mut self, level:usize) -> bool{
        //log::info!("New Line {} {} {}", self.level, level, self.ready_line);
        if level > self.level && !self.ready_line {
            return false;
        }
        let current_state = self.state.last().unwrap().to_owned();
        match current_state {
            State::CHead | State::DHead => {
                self.replace_head(State::Body);
            }
            _ => {
                //log::info!("Popping {} {} {}", level, self.level, self.context.len());
                for _ in level..self.context.len()-2 {
                    self.context.pop_context();
                    self.state.pop();
                }
            }
        }
        return true;
    }

    pub fn get_position(&mut self, sp:&str) -> (u32,u32) {
        let position = match self.ident_state {
            IdentState::Local => { // Write to the local Context
                self.context.put_local(sp)
            }
            IdentState::Read =>  {
                if self.context.len() <= 3 {
                    self.context.get_or_global(sp)
                }
                else {
                    self.context.get_or_put_local(sp)
                }   
            }
            IdentState::Write =>  {
                if self.context.len() <= 2 {
                    self.context.get_or_global(sp)
                }
                else {
                    self.context.get_or_put_local(sp)
                }   
            } 
        };
        position
    }

    pub fn check_ident(&self, text:&str) -> bool{
        let mut last = false;
        for c in text.chars() {
            let current = c.is_uppercase();
            if last && current {
                return false;
            }
            last = current;
        }
        return true;
    }

    pub fn put_token(&mut self, token:Token, text:&str, level:usize) -> Option<TokenResult> {
        
        self.ready_line = token == Token::SymbolColon;
        let result = match token {
            Token::Ident => {
                if !self.check_ident(text) {
                    //log::info!("Failed {}", text);
                    TokenResult { token: token.clone(), level: level, position: None, text: Some(text.to_string()) }
                }
                else {
                    let split = text.split("_");
                    let positions:Vec<(u32,u32)> = split.map(|s| self.get_position(s.to_lowercase().as_str())).collect();
                    
                    TokenResult { token: token.clone(), level: level, position: Some(positions), text: Some(text.to_string()) }
                }
            }
            _ => {
                TokenResult { token: token.clone(), level: level, position: None, text:None }
            }
        };
        
        if self.state.len() == 0 {
            return None;
        }

        match self.state.last().to_owned().unwrap() {
            State::Root | State::Body => {
                match token {
                    Token::KeyDef => {
                        self.state.push(State::DParam);
                        self.ident_state = IdentState::Write;
                    },
                    Token::KeyClass | Token::KeyIf | Token::KeyElse | Token::KeyElif | Token::KeyWhile | Token::KeyFor 
                    | Token::KeyTry | Token::KeyExcept | Token::KeyWith | Token::KeyFinally => {
                        self.state.push(State::CHead);
                        self.context.push_context();
                        self.ident_state = IdentState::Read;
                    }
                    _ => {}
                }
            }
            State::DParam => {
                self.replace_head(State::DHead);
                self.context.push_context();
            }
            State::DHead => { // Local Variables in header global otherwise
                match token {
                    Token::Ident => {self.ident_state = IdentState::Local;}
                    Token::SymbolLeftParen => {self.ident_state = IdentState::Local;}
                    Token::SymbolColon => {self.ident_state = IdentState::Read;}
                    Token::SymbolArrow => {self.ident_state = IdentState::Read;}
                    _ => {}
                }
            }
            State::CHead => {
                match token {
                    Token::Ident => {self.ident_state = IdentState::Read;}
                    Token::SymbolColon => {self.replace_head(State::Body); self.ident_state = IdentState::Read;}
                    _ => {}
                }
            }
        }
        self.level = level;
        
        //log::info!("S {:?} {:?}", self.state, self.ident_state);
        //if result.token == Token::Ident {
        //log::info!("T {:?}", result);
        //}
        

        Some(result)
        
        
    }
}

