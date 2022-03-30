use crate::{syntax,lexer};
use lalrpop_util::lalrpop_mod;
lalrpop_mod!(pub grammar);

pub fn parse (source : &str) -> Result<syntax::Cmd, String> {
    grammar::SeqParser::new().parse(lexer::Lexer::new(source))
	.map_err(|err| format!("{:?}",err))
}
