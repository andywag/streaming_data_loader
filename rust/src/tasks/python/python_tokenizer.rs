

use logos::Logos;
use serde::{Deserialize, Serialize};

use super::{context_map::{ContextLookup, ContextStore}, python_parser::{StateMachine, TokenResult}};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Operator {
  At,
  Not,
  Assign,
  Colon,
  PlusEqual,
  MinusEqual,
  PlusDoubleColon,
  Mul,
  Div,
  Mod,
  Plus,
  Minus,
  ShiftLeft,
  ShiftRight,
  LessThan,
  GreaterThan,
  LessThanOrEqual,
  GreaterThanOrEqual,
  Equal,
  NotEqual,
  BitAnd,
  BitXor,
  BitOr,
  BitNeg,
  And,
  Or,
  Unknown,
}

impl Operator {
  fn from_str(s: &str) -> Operator {
    match s {
      "@" => Operator::At,
      "!" => Operator::Not,
      "=" => Operator::Assign,
      ":" => Operator::Colon,
      "+=" => Operator::PlusEqual,
      "-=" => Operator::MinusEqual,
      "*" => Operator::Mul,
      "/" => Operator::Div,
      "%" => Operator::Mod,
      "+" => Operator::Plus,
      "-" => Operator::Minus,
      "<<" => Operator::ShiftLeft,
      ">>" => Operator::ShiftRight,
      "<" => Operator::LessThan,
      ">" => Operator::GreaterThan,
      "<=" => Operator::LessThanOrEqual,
      ">=" => Operator::GreaterThanOrEqual,
      "==" => Operator::Equal,
      "!=" => Operator::NotEqual,
      "&" => Operator::BitAnd,
      "^" => Operator::BitXor,
      "|" => Operator::BitOr,
      "~" => Operator::BitNeg,
      "&&" => Operator::And,
      "||" => Operator::Or,
      _ => Operator::Unknown,
    }
  }

  pub fn get_id(&self, start:u32) -> u32 {
    match self {
        Operator::At => start,
        Operator::Not => start+1,
        Operator::Assign => start+2,
        Operator::Colon => start+3,
        Operator::PlusEqual => start+4,
        Operator::MinusEqual => start+5,
        Operator::PlusDoubleColon => start+6,
        Operator::Mul => start+8,
        Operator::Div => start+9,
        Operator::Mod => start+10,
        Operator::Plus => start+11,
        Operator::Minus => start+12,
        Operator::ShiftLeft => start+13,
        Operator::ShiftRight => start+14,
        Operator::LessThan => start+15,
        Operator::GreaterThan => start+16,
        Operator::LessThanOrEqual => start+17,
        Operator::GreaterThanOrEqual => start+18,
        Operator::Equal => start+19,
        Operator::NotEqual => start+20,
        Operator::BitAnd => start+21,
        Operator::BitXor => start+22,
        Operator::BitOr => start+23,
        Operator::BitNeg => start+24,
        Operator::And => start+25,
        Operator::Or => start+26,
        Operator::Unknown => start+27,
    }
  }
}

pub(super) fn lex_operator<'a>(lex: &mut logos::Lexer<'a, Token>) -> Operator {
  Operator::from_str(lex.slice())
}


#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {

    #[token("False")]
    KeyFalse,
    #[token("None")]
    KeyNone, 
    #[token("True")]
    KeyTrue,
    #[token("and")]
    KeyAnd,
    #[token("as")]
    KeyAs,
    #[token("assert")]
    KeyAssert,
    #[token("async")]
    KeyAsync,
    #[token("await")]
    KeyAwait,
    #[token("break")]
    KeyBreak,
    #[token("class")]
    KeyClass,
    #[token("continue")]
    KeyContinue,
    #[token("def")]
    KeyDef,
    #[token("del")]
    KeyFrom,
    #[token("from")]
    KeyDel,
    #[token("elif")]
    KeyElif,
    #[token("else")]
    KeyElse,
    #[token("except")]
    KeyExcept,
    #[token("finally")]
    KeyFinally,
    #[token("for")]
    KeyFor,
    #[token("global")]
    KeyGlobal,
    #[token("if")]
    KeyIf,
    #[token("import")]
    KeyImport,
    #[token("in")]
    KeyIn,
    #[token("is")]
    KeyIs,
    #[token("lambda")]
    KeyLambda,
    #[token("nonlocal")]
    KeyNonLocal,
    #[token("not")]
    KeyNot,
    #[token("or")]
    KeyOr,
    #[token("pass")]
    KeyPass,
    #[token("raise")]
    KeyRaise,
    #[token("return")]
    KeyReturn,
    #[token("try")]
    KeyTry,
    #[token("with")]
    KeyWith,
    #[token("while")]
    KeyWhile,

    // Tokens can be literal strings, of any length.
    #[token("\\")]
    SymbolContinue,

    #[token("{")]
    SymbolLeftBrace,
  
    #[token("}")]
    SymbolRightBrace,
  
    #[token("[")]
    SymbolLeftBracket,
  
    #[token("]")]
    SymbolRightBracket,
  
    #[token(",")]
    SymbolComma,
  
    #[token(".")]
    SymbolDot,
  
    #[token("(")]
    SymbolLeftParen,
  
    #[token(")")]
    SymbolRightParen,
  
    #[token(":")]
    SymbolColon,
  
    #[token("$")]
    SymbolDollar,

    #[token("->")]
    SymbolArrow,

    // Or regular expressions.
    #[regex(r"'[^']*'")]
    String,
    #[regex("\"(?s:[^\"\\\\]|\\\\.)*\"")]
    AString,
    #[regex(r#"""[^"""]""""#)]
    TString,
    #[regex(r"[_a-zA-Z][_a-zA-Z0-9]*")]
    Ident,
    #[regex(r"(?:0|[1-9][0-9]*)(?:\.[0-9]+)?(?:[eE][+-]?[0-9]+)?")]
    Number,
    #[regex(r"0x(?:0|[1-9][0-9]*)")]
    HexNumber,
    #[regex(r"[@!\$~\+\-&\|\^=<>\*/%]+", lex_operator)]
    Op(Operator),
    #[regex(r"#[^\r\n]*(\r\n|\n)?")]
    Comment,
    // We can also use this variant to define whitespace,
    // or any other matches we wish to skip.
    //#[regex(r"[ \t\f]+", logos::skip)]
    #[regex(r"[ ]+")]
    WS,
    #[regex(r"[\t]+")]
    Tab,
    #[regex(r"[\n]")]
    Newline,

    // Logos requires one token variant to handle errors,
    // it can be named anything you wish.
    #[error]
    Error,


    Root
}



pub fn get_token_id(token:Token) -> u32{
    let start:u32 = 100;
    match token {

        Token::Comment => 90,
        Token::WS => 91,
        Token::Tab => 92,
        Token::Newline => 93,
        Token::Error => 94,
        Token::Root => 95,
        Token::KeyFalse => start,
        Token::KeyNone => start + 1,
        Token::KeyTrue => start + 2,
        Token::KeyAnd =>  start + 3,
        Token::KeyAs => start + 4,
        Token::KeyAssert => start + 5,
        Token::KeyAsync => start + 6,
        Token::KeyAwait => start + 7,
        Token::KeyBreak => start + 8,
        Token::KeyClass => start + 9,
        Token::KeyContinue => start + 10,
        Token::KeyDef => start + 11,
        Token::KeyDel => start + 12,
        Token::KeyElif => start + 13,
        Token::KeyElse => start + 14,
        Token::KeyExcept => start + 15,
        Token::KeyFinally => start + 16,
        Token::KeyFor => start + 17,
        Token::KeyGlobal => start + 18,
        Token::KeyIf => start + 19,
        Token::KeyImport => start + 20,
        Token::KeyIn => start + 21,
        Token::KeyIs => start + 22,
        Token::KeyLambda => start + 23,
        Token::KeyNonLocal => start + 24,
        Token::KeyNot => start + 25,
        Token::KeyOr => start + 26,
        Token::KeyPass => start + 27,
        Token::KeyRaise => start + 28,
        Token::KeyReturn => start + 29,
        Token::KeyTry => start + 30,
        Token::KeyWhile => start + 31,
        Token::KeyFrom => start + 32,
        Token::KeyWith => start + 33,
        Token::SymbolLeftBrace => start + 34,
        Token::SymbolRightBrace => start + 35,
        Token::SymbolLeftBracket => start + 36,
        Token::SymbolRightBracket => start + 37,
        Token::SymbolComma => start + 38,
        Token::SymbolDot => start + 39,
        Token::SymbolLeftParen => start + 40,
        Token::SymbolRightParen => start + 41,
        Token::SymbolColon => start + 42,
        Token::SymbolDollar => start + 43,
        Token::SymbolContinue => start + 44,
        Token::SymbolArrow => start + 45,
        Token::String | Token::AString | Token::TString => start + 46,
        Token::Ident => start + 47,
        Token::Number => start + 48,
        Token::HexNumber => start + 49,


        Token::Op(x) => x.get_id(start+49),
        
    }

}

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


pub fn lex_simple<'a>(text:&str, global_store:&'a mut ContextLookup) -> Option<Vec<TokenResult>> {
    if !check_python(text) {
        return None;
    }

    
    let mut tokens = Vec::<TokenResult>::with_capacity(1024);

    let mut lexer = Token::lexer(text);

    let mut level = 0;
    let mut indent_width:Option<usize> = None;

    let local_store = &mut ContextLookup::new(1024);
    let context = ContextStore::new(global_store, local_store);
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
    //log::info!("Data Length :  {}", tokens.len());
    if tokens.len() > 64 {
        return Some(tokens);
    }
    else {
        return None;
    }

    
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PythonTokenizer {
    global_store:ContextLookup,
    index:u32
}

impl PythonTokenizer {
    pub fn new(s:usize) -> Self {
        Self {
            global_store:ContextLookup::new(s),
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
        for token in tokens_opt.unwrap() {
            if token.token == Token::Ident {
                match token.position {
                    Some(p) => {
                        /*for x in p {
                            ids.push(x.0 as u32 + 10);
                            ids.push(x.1 as u32 + 200);
                        }*/
                        ids.push(p[0].0 as u32 + 10);
                    },
                    None => {},
                }
            }
            else {
                ids.push(get_token_id(token.token));
            }
        }
        ids
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

#[test]
pub fn test_file() {
    use std::fs::File;
    use std::io::Read;

    crate::create_logger();

    let mut global_store = IdentLookup::new(1024);

    let mut file = File::open("temp.py").unwrap();
    let mut contents = String::new();
    let _= file.read_to_string(&mut contents);
   

    let _tokens = lex_simple(contents.as_str(), &mut global_store);
    //for token in _tokens {
    //    println!("Token: {:?}", token);
    //}
    
}*/


