

use logos::Logos;


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
    #[token("from")]
    KeyFrom,
    #[token("del")]
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
    #[regex(r"[_a-zA-Z][_a-zA-Z0-9.]*")]
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

impl Token {
    pub fn get_token_id(&self) -> u32{
        let start:u32 = 100;
        match self {
    
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
    
    
            Token::Op(x) => x.get_id(200),
            
        }
    
    }
}


pub fn check_python(text:&str) -> bool{
  let mut lexer = Token::lexer(text);
  let mut key_count = 0;
  while let Some(token) = lexer.next() {
      match token {
           Token::KeyImport | Token::KeyDel | Token::KeyDef | Token::KeyElif | Token::KeyExcept => {key_count += 1;}
          _ => {}
      }
      if key_count > 8 {
          return true;
      }
  }
  return false;
}
