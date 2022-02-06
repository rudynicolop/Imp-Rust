use std::{env,fs};
mod lib;

fn main() -> Result<(),String> {
    let args : Vec<String> = env::args().collect();
    match fs::read_to_string(&args[0]) {
	Ok(prog) => {
	    let ast : syntax::cmd =
		imp::grammar::SeqParser::new().parse(prog)?;
	    ast.eval(bigstep::Store::new())
	},
	Err(err) => Err(String::from("problem"))
    }
}
