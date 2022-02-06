use std::{env,fs};
use imp::{grammar,bigstep};

fn main() -> Result<(),String> {
    let args : Vec<String> = env::args().collect();
    match fs::read_to_string(&args[1]) {
	Ok(prog) => {
	    match grammar::SeqParser::new().parse(&prog) {
		Ok(ast) => {
		    println!("Program parsed as");
		    println!("{}",ast);
		    println!("Executing program");
		    ast.eval(&mut bigstep::Store::new())
		}
		Err(err)  => {
		    println!("{}",err);
		    Err(String::from("Failed to parse."))
		}
	    }
	},
	Err(err) => {
	    println!("{}",err);
	    Err(String::from("File not found."))
	}
    }
}
