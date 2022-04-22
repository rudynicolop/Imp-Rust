use std::{char, fmt, sync::Arc, str::CharIndices, iter::{Iterator, Peekable}};
use peeking_take_while::PeekableExt;
use codespan::{FileMap, ByteIndex, ByteOffset, LineIndex, ColumnIndex};

#[derive(Clone,Debug)]
pub enum Token {
    LBRACE,
    RBRACE,
    LPAREN,
    RPAREN,
    SEMICOLON,
    IF,
    ELSE,
    WHILE,
    ASGN,
    PRINT,
    SKIP,
    OR,
    AND,
    EQ,
    LT,
    ADD,
    SUB,
    MUL,
    BOOL(bool),
    NUM(i32),
    VAR(String)
}

impl Token {
    /// Lexical length of a token.
    fn len(&self) -> usize {
	use Token::*;
	match self {
	    LBRACE |
	    RBRACE |
	    LPAREN |
	    RPAREN |
	    ADD    |
	    SUB    |
	    MUL    |
	    SEMICOLON => 1,
	    OR  |
	    EQ  |
	    LT  |
	    IF  |
	    ASGN => 2,
	    AND  => 3,
	    ELSE |
	    SKIP |
	    BOOL (true) => 4,
	    WHILE |
	    PRINT |
	    BOOL (false) => 5,
	    NUM(z) => z.to_string().len(),
	    VAR(x) => x.len()
	}
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	use Token::*;
	match self {
	    LBRACE    => write!(f,"{}","{"),
	    RBRACE    => write!(f,"{}","}"),
	    LPAREN    => write!(f,"("),
	    RPAREN    => write!(f,")"),
	    SEMICOLON => write!(f,";"),
	    IF        => write!(f,"if"),
	    ELSE      => write!(f,"else"),
	    WHILE     => write!(f,"while"),
	    ASGN      => write!(f,":="),
	    PRINT     => write!(f,"print"),
	    SKIP      => write!(f,"skip"),
	    OR        => write!(f,"or"),
	    AND       => write!(f,"and"),
	    EQ        => write!(f,"=?"),
	    LT        => write!(f,"<?"),
	    ADD       => write!(f,"+"),
	    SUB       => write!(f,"-"),
	    MUL       => write!(f,"*"),
	    BOOL(b)   => write!(f,"{}",b),
	    NUM(z)    => write!(f,"{}",z),
	    VAR(x)    => write!(f,"{}",x)
	}
    }
}

fn token_of_string (s : &str) -> Token {
    match s {
	"if"    => Token::IF,
	"else"  => Token::ELSE,
	"while" => Token::WHILE,
	"print" => Token::PRINT,
	"skip"  => Token::SKIP,
	"and"   => Token::AND,
	"or"    => Token::OR,
	"true"  => Token::BOOL (true),
	"false" => Token::BOOL (false),
	_       => Token::VAR (s.to_string())
    }
}

pub type Spanned = (
    ByteIndex, // start of token in file
    Token,
    ByteIndex  // end of token in file
);

/// Lexical errors.
#[derive(Debug)]
pub enum BadLex {
    NonTokenChar(char),
    ExpectedChar(char,char),
    Internal(std::num::ParseIntError)
}

impl fmt::Display for BadLex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	use BadLex::*;
	match self {
	    NonTokenChar(c) => write!(f,"No valid token includes {}.",c),
	    ExpectedChar(c1,c2) => write!(f,"After {} expected {}.",c1,c2),
	    Internal(err) => write!(f,"Implemtation error: {}.",err)
	}
    }
}

pub struct Lexer<'input> {
    chars : Peekable<CharIndices<'input>> // characters to lex.
}

impl<'input> Lexer<'input> {
    pub fn new(source: &'input FileMap) -> Self {
        Lexer { chars: source.src().char_indices().peekable() }
    }
}

pub fn spanned (i : usize, token : Token) -> Spanned {
    let offset = token.len() as i64;
    (ByteIndex(i as u32), token,
     ByteIndex(i as u32) + ByteOffset (offset))
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Spanned,(ByteIndex,BadLex)>;
    
    fn next(&mut self) -> Option<Self::Item> {
	loop {
	    if let Some((i,c)) = self.chars.next() {

		// Skip whitespace.
		if c.is_whitespace() { continue }
		
		return Some (match c {
		    '{' => Ok (spanned (i,Token::LBRACE)),
		    '}' => Ok (spanned (i,Token::RBRACE)),
		    '(' => Ok (spanned (i,Token::LPAREN)),
		    ')' => Ok (spanned (i,Token::RPAREN)),
		    ';' => Ok (spanned (i,Token::SEMICOLON)),
		    '+' => Ok (spanned (i,Token::ADD)),
		    '*' => Ok (spanned (i,Token::MUL)),
		    '-' => Ok (spanned (i,Token::SUB)),
		    ':' => {
			if let Some((_,'=')) = self.chars.next() {
			    Ok (spanned (i,Token::ASGN))
			} else {
			    Err ((ByteIndex(i as u32),
				  BadLex::ExpectedChar (':','='))) }
		    },
		    '<' => {
			if let Some((_,'?')) = self.chars.next() {
			    Ok (spanned (i,Token::LT))
			} else {
			    Err ((ByteIndex(i as u32),
				  BadLex::ExpectedChar ('<','?'))) }
		    },
		    '=' => {
			if let Some((_,'?')) = self.chars.next() {
			    Ok (spanned (i,Token::EQ))
			} else {
			    Err ((ByteIndex(i as u32),
				  BadLex::ExpectedChar ('=','?'))) }
		    },
		    _ => {
			if c.is_ascii_digit() {
			    let mut num = c.to_string();
			    self.chars
				.by_ref()
				.peeking_take_while(|ch| ch.1.is_ascii_digit())
				.for_each(|ch| num.push(ch.1));
			    num.parse::<i32>()
				.map_err
				(|err| (ByteIndex (i as u32),
					BadLex::Internal (err)))
				.map (|z| spanned (i, Token::NUM(z)))
			} else if c.is_alphabetic() {
			    let mut s = c.to_string();
			    self.chars
				.by_ref()
				.peeking_take_while(|ch| ch.1.is_alphanumeric())
				.for_each(|ch| s.push(ch.1));
			    Ok (spanned (i, token_of_string(&s)))
			} else {
			    Err ((ByteIndex(i as u32),
				  BadLex::NonTokenChar (c)))
			}
		    }
		})
	    } else { return None }
	}
    }
}

pub fn tokenize(src : Arc<FileMap>)
	    -> Result<Vec::<Spanned>, (u32, u32, BadLex)> {
    let mut tokens = Vec::new();
    let mut lexer = Lexer::new(&src);
    while let Some (result) = lexer.next() {
	match result {
	    Ok (sp) => tokens.push(sp),
	    Err ((index, err)) => {
		let (LineIndex (row), ColumnIndex (col))
		    = src.location(index)
		    .expect("Looking up bad index in file");
		return Err ((row, col, err))
	    }
	}
    }
    Ok (tokens)
}
