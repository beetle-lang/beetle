extern crate thiserror;

use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LexerError {
    #[error("IO Error")]
    FileIO(#[from] io::Error),

    #[error("Was expecting {expected:?}, found {found:?}")]
    MissingExpectedSymbol {
        expected: TokenType,
        found: Token
    }
}

pub type Token = TokenType;

pub struct Punctuation {
    raw: char,
    kind: PunctuationKind
}

#[derive(Debug)]
pub enum TokenType {
    /* End of Token Stream */
    EOF,

    /* Punctaution like, ( [ */
    Punctaution{raw: char, kind: PunctuationKind},

    /* Operators are 'actions' that you take on an entities i.e. '*', '->' */
    Operators(String),

    /* A sequence of characters */
    Identifier(String),

    /* A single character 'a' => unicode codepoint (32 bits)*/
    Char(char),

    /*  */
    Numeric(String),

    /* For errors */
    Unknown(char),
}

#[derive(Debug)]
pub enum PunctuationKind{
    /* '{' and '[' */
    Open(usize),
    
    /* '}' and ']' */
    Close(usize),

    /* ',' and ':' */
    Separator,
}

type BalancingDepthType = i32;

pub struct Lexer {
    /* Human Readable positions in file */
    pub cur_line: usize,
    pub cur_cel: usize,

    /* 'raw' format / offset within the file (in terms of 'codepoints') */
    pub codepoint_offset: usize,

    chars: std::iter::Peekable<std::str::Chars<'a>>,
    balancing_state: std::collections::hash_map<char, BalancingDepthType>,
}

impl<'a> Lexer<'a>{
    pub fn new(chars: &'a str) -> Lexer<'a> {
        Lexer { 
            cur_line: 1, 
            cur_cel: 1, 
            
            codepoint_offset: 0, 
            
            chars: chars.chars().peekable(), 
            balancing_state: std::collections::hash_map::new()
        }
    }

    fn push_open(&mut self, c: char) -> BalancingDepthType {
        if let Some(v) = self.balancing_state.contains_key(&c){
            *v += 1;
            *v
        } else {
            self.balancing_state.insert(*c, 0);
            0
        }
    }

    fn transform_to_type(c: char) -> Option<TokenType> {
        match c {
            '(' => Some(TokenType::Punctaution { raw: c, kind: PunctuationKind::Open(0) }),
            ')' => Some(TokenType::Punctaution { raw: c, kind: PunctuationKind::Close(0) }),
        }
    }
}