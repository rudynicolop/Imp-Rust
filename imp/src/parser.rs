use crate::{syntax,lexer};
use lalrpop_util::lalrpop_mod;
lalrpop_mod!(pub grammar);

pub fn parse (tokens : Vec<lexer::Spanned>) -> Result<syntax::Cmd, String> {
    grammar::SeqParser::new().parse(tokens)
	.map_err(|err| format!("{:?}",err))
}
