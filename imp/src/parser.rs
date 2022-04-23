use crate::{syntax,lexer::{Token,Spanned,BadLex}};
use std::sync::Arc;
use codespan::{FileMap, LineIndex, ColumnIndex, ByteIndex};
use lalrpop_util::{lalrpop_mod, ParseError};
lalrpop_mod!(pub grammar);

fn handle_parse_error<'a>
    (src : &'a Arc<FileMap>)
     -> impl FnOnce(ParseError<ByteIndex,Token,BadLex>) -> () + 'a {
	move |err : ParseError<ByteIndex,Token,BadLex>| {
	    println!("Parsing error:");
	    match err {
		ParseError::InvalidToken {location:index} => {
		    let (LineIndex(row), ColumnIndex(col))
			= src.location(index)
			.expect("Looking up bad index in file for invalid token.");
		    println!("Invalid token at line {} column {}.",row,col);
		}
		ParseError::UnrecognizedEOF {location:index, expected:exp} => {
		    let (LineIndex(row), ColumnIndex(col))
			= src.location(index)
			.expect("Looking up bad index in file for unrecognized eof.");
		    println!(
			"Unrecognized end of file at line {} column {}, expected {:?}.",
			row,col,exp);
		}
		ParseError::UnrecognizedToken {token:(i1,tk,i2), expected:exp} => {
		    let (LineIndex(r1), ColumnIndex(c1))
			= src.location(i1)
			.expect("Looking up bad index in file for unrecognized token's start.");
		    let (LineIndex(r2), ColumnIndex(c2))
			= src.location(i2)
			.expect("Looking up bad index in file for unrecognized token's start.");
		    println!(
			"Unrecognized token {} at line {} column {} to line {} column {}, expected {:?}.",
			tk,r1,c1,r2,c2,exp);
		}
		ParseError::ExtraToken {token:(i1,tk,i2)} => {
		    let (LineIndex(r1), ColumnIndex(c1))
			= src.location(i1)
			.expect("Looking up bad index in file for extra token's end.");
		    let (LineIndex(r2), ColumnIndex(c2))
			= src.location(i2)
			.expect("Looking up bad index in file for extra token's end.");
		    println!(
			"Extra token {} at line {} column {} to line {} column {}.",
			tk,r1,c1,r2,c2);
		}
		ParseError::User {error:err} => println!("Lexical error: {}",err)
	    }
	}
    }

pub fn parse (src : &Arc<FileMap>, tokens : Vec<Spanned>) -> Result<syntax::Cmd, ()> {
    grammar::SeqParser::new().parse(tokens).map_err(handle_parse_error(src))
}
