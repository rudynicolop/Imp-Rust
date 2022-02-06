use std::{env,fs};
use imp::{grammar,bigstep};

fn main() -> Result<(),String> {
    let args : Vec<String> = env::args().collect();
    match fs::read_to_string(&args[1]) {
	Ok(prog) => {
	    match grammar::SeqParser::new().parse(&prog) {
		Ok(ast) => ast.eval(&mut bigstep::Store::new()),
		Err(_)  => Err(String::from("Could not parse program."))
	    }
	},
	Err(_) => Err(String::from("Could not read file."))
    }
}
