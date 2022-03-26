use std::{fmt, str::CharIndices, iter::Iterator};

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

// The following is stolen from the [LALRPOP] tutorial.

pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

// Lexical errors.
pub enum Error {
    NoToken(String)
}

pub struct Lexer<'input> {
    chars : CharIndices<'input>; // token stream
    // next : Option(usize,char) // next character
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
	let stream := input.char_indices();
        Lexer { chars: input.char_indices();
		//next: stream.next()
	}
    }

    /*
    pub fn peek(&self) -> Option<usize,char> {
	self.next()
    }
     */

    fn match_token_helper(&self,pair : (usize,char)) -> Option(usize,Token,usize) {
	let i = pair.0;
	match pair.1 {
	    '{' => Some ((i,Token::LBRACE,i+1)),
	    '}' => Some ((i,Token::RBRACE,i+1)),
	    '(' => Some ((i,Token::LPAREN,i+1)),
	    ')' => Some ((i,Token::RPAREN,i+1)),
	    ';' => Some ((i,Token::SEMICOLON,i+1)),
	    '+' => Some ((i,Token::ADD,i+1)),
	    '*' => Some ((i,Token::MUL,i+1)),
	    '-' => Some ((i,Token::SUB,i+1)),
	    ':' =>
		match self.chars().next() {
		    Some ((_,'=')) => Some ((i,Token::ASGN,i+2)),
		    None           => None },
	    '=' =>
		match self.chars().next() {
		    Some ((_,'?')) => Some ((i,Token::EQ,i+2)),
		    None           => None },
	    '<' =>
		match self.chars().next() {
		    Some ((_,'?')) => Some ((i,Token::LT,i+2)),
		    None           => None },
	    'i' =>
		match self.chars().next() {
		    Some ((_,'f')) => Some ((i,Token::IF,i+2)),
		    None           => None },
	    'o' =>
		match self.chars().next() {
		    Some ((_,'r')) => Some ((i,Token::OR,i+2)),
		    None           => None },
	    'a' =>
		match self.chars().next() {
		    Some ((_,'n')) =>
			match chars().next() {}
			Some ((i,Token::IF,i+2)),
		    None           => None },
	    
	}
    }
    
    /// Returns an optional end index and symbol token.
    /// Advances the cursor.
    pub fn match_token(&self) -> Option<(usize,Token,usize)> {
	self.chars().next().and_then
	    Some((i,'{')) => Some ((i,Token::LBRACE,i+1)),
	    Some((i,'}')) => Some ((i,Token::RBRACE,i+1)),
	    Some((i,'(')) => Some ((i,Token::LPAREN,i+1)),
	    Some((i,')')) => Some ((i,Token::RPAREN,i+1)),
	    Some((i,';')) => Some ((i,Token::SEMICOLON,i+1)),
	    Some((i,'+')) => Some ((i,Token::ADD,i+1)),
	    Some((i,'*')) => Some ((i,Token::MUL,i+1)),
	    Some((i,'-')) => Some ((i,Token::SUB,i+1)),
	Some((i,':')) =>
	    match self.chars().next() {
		    Some ((_,'=')) => Some ((i,Token::ASGN,i+2)),
		    None           => None
		}
	}
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Spanned<Token, usize, Error>;
    
    fn next(&mut self) -> Option<Self::Item> {
	loop {
	    match self.chars.peek() {
		None          => return None, // EOF
		Some((_, ' ')) |
		Some((_, '\n')) |
		Some((_, '\t')) => continue, // whitespace
		Some((i,'{')) => Some ((i,Token::LBRACE,i+1)),
		Some((i,'}')) => Some ((i,Token::RBRACE,i+1)),
		Some((i,'(')) => Some ((i,Token::LPAREN,i+1)),
		Some((i,')')) => Some ((i,Token::RPAREN,i+1)),
		Some((i,';')) => Some ((i,Token::SEMICOLON,i+1)),
		Some((i,'+')) => Some ((i,Token::ADD,i+1)),
		Some((i,'*')) => Some ((i,Token::MUL,i+1)),
		Some((i,'-')) => Some ((i,Token::SUB,i+1)),
		Some()
		_ => continue
	    }
	}
    }
}
