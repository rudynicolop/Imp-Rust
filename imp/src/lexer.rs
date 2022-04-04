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

pub struct Info <A> {
    line_no : usize,   // line number
    column_no : usize, // column start number
    item : A
}

impl <A> Info <A> {
    pub fn new(line_no:usize, col_no:usize, a:A) -> Self {
	Info { line_no:line_no, column_no:col_no, item:a }
    }
}

/// Lexical errors.
#[derive(Debug)]
pub enum BadLex {
    NonTokenChar(char),
    ExpectedChar(char,char),
    Internal(std::num::ParseIntError)
}

pub struct Lexer<'input> {
    chars : Peekable<CharIndices<'input>>,
    curr_line_no : usize // current line.
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Lexer { chars: input.char_indices().peekable(),
		curr_line_no: 0 }
    }

    pub fn new_info <A> (self, col_no:usize, a:A) -> Info <A> {
	Info::new
	    (self.curr_line_no + 1,
	     col_no - self.curr_line_no, a)
    }
}

pub type Spanned = Result<Info<Token>, Info<BadLex>>;

impl<'a> Iterator for Lexer<'a> {
    type Item = Spanned;
    
    fn next(&mut self) -> Option<Self::Item> {
	loop {
	    if let Some((i,c)) = self.chars.next() {

		// Check whitespace.
		if c.is_whitespace() {
		    if let '\n' = c {
			// Increment line number
			// when encountering a newline.
			self.curr_line_no = self.curr_line_no + 1
		    }
		    continue
		}
		
		return Some (match c {
		    '{' => Ok (self.new_info(i,Token::LBRACE)),
		    '}' => Ok (self.new_info(i,Token::RBRACE)),
		    '(' => Ok (self.new_info(i,Token::LPAREN)),
		    ')' => Ok (self.new_info(i,Token::RPAREN)),
		    ';' => Ok (self.new_info(i,Token::SEMICOLON)),
		    '+' => Ok (self.new_info(i,Token::ADD)),
		    '*' => Ok (self.new_info(i,Token::MUL)),
		    '-' => Ok (self.new_info(i,Token::SUB)),
		    ':' => {
			if let Some((_,'=')) = self.chars.next() {
			    Ok (self.new_info(i,Token::ASGN))
			} else {
			    Err (self.new_info
				 (i, BadLex::ExpectedChar (':','=')))
			}
		    },
		    '<' => {
			if let Some((_,'?')) = self.chars.next() {
			    Ok (self.new_info(i,Token::LT))
			} else {
			    Err (self.new_info
				 (i, BadLex::ExpectedChar ('<','?')))
			}
		    },
		    '=' => {
			if let Some((_,'?')) = self.chars.next() {
			    Ok (self.new_info(i,Token::EQ))
			} else {
			    Err (self.new_info
				 (i, BadLex::ExpectedChar ('=','?')))
			}
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
				(|err|
				 self.new_info(i, BadLex::Internal (err)))
				.map
				(|z|
				 self.new_info(i, Token::NUM(z)))
			} else if c.is_alphabetic() {
			    let mut s = c.to_string();
			    self.chars
				.by_ref()
				.peeking_take_while(|ch| ch.1.is_alphanumeric())
				.for_each(|ch| s.push(ch.1));
			    Ok (self.new_info(i,token_of_string(&s)))
			} else {
			    Err (self.new_info(i,BadLex::NonTokenChar(c)))
			}
		    }
		})
	    } else { return None }
	}
    }
}
