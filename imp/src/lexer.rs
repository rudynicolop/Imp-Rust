use std::{char, fmt, str::CharIndices, iter::{Iterator, Peekable}};
use peeking_take_while::PeekableExt;

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

// The following is stolen from the [LALRPOP] tutorial.

pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

/// Lexical errors.
#[derive(Debug)]
pub enum BadLex {
    NonTokenChar(usize,char),
    ExpectedChar(usize,char,char),
    Internal(usize,std::num::ParseIntError)
}

pub struct Lexer<'input> {
    chars : Peekable<CharIndices<'input>>
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Lexer { chars: input.char_indices().peekable() }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Spanned<Token, usize, BadLex>;
    
    fn next(&mut self) -> Option<Self::Item> {
	loop {
	    if let Some((i,c)) = self.chars.next() {
		// Check whitespace.
		if c.is_whitespace() { continue }
		
		return Some (match c {
		    '{' => Ok ((i,Token::LBRACE,i+1)),
		    '}' => Ok ((i,Token::RBRACE,i+1)),
		    '(' => Ok ((i,Token::LPAREN,i+1)),
		    ')' => Ok ((i,Token::RPAREN,i+1)),
		    ';' => Ok ((i,Token::SEMICOLON,i+1)),
		    '+' => Ok ((i,Token::ADD,i+1)),
		    '*' => Ok ((i,Token::MUL,i+1)),
		    '-' => Ok ((i,Token::SUB,i+1)),
		    ':' => {
			if let Some((_,'=')) = self.chars.next() {
			    Ok ((i,Token::ASGN,i+2))
			} else {
			    Err (BadLex::ExpectedChar (i,':','='))
			}
		    },
		    '<' => {
			if let Some((_,'?')) = self.chars.next() {
			    Ok ((i,Token::LT,i+2))
			} else {
			    Err (BadLex::ExpectedChar (i,'<','?'))
			}
		    },
		    '=' => {
			if let Some((_,'?')) = self.chars.next() {
			    Ok ((i,Token::EQ,i+2))
			} else {
			    Err (BadLex::ExpectedChar (i,'=','?'))
			}
		    },
		    _ => {
			if c.is_ascii_digit() {
			    let mut num = c.to_string();
			    self.chars
				.by_ref()
				// .peekable()
				.peeking_take_while(|ch| ch.1.is_ascii_digit())
				.for_each(|ch| num.push(ch.1));
			    num.parse::<i32>()
				.map_err(|err| BadLex::Internal(i,err))
				.map(|z| (i,Token::NUM(z),i + num.len()))
			} else if c.is_alphabetic() {
			    let mut s = c.to_string();
			    self.chars
				.by_ref()
				// .peekable()
				.peeking_take_while(|ch| ch.1.is_alphanumeric())
				.for_each(|ch| s.push(ch.1));
			    Ok ((i,token_of_string(&s),i + s.len()))
			} else {
			    Err (BadLex::NonTokenChar(i,c))
			}
		    }
		})
	    } else { return None }
	}
    }
}
