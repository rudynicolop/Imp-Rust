use std::{env,fs};
use imp::{grammar,bigstep,syntax};

fn main() -> Result<(),()> {
    let args : Vec<String> = env::args().collect();
    let prog = fs::read_to_string(&args[1])
	.map_err(|err| println!("File error: {}",err))?;
    let ast: syntax::Cmd =
	grammar::SeqParser::new().parse(&prog)
	.map_err(|err| println!("Parsing error: {}",err))?;
    println!("Program parsed as");
    println!("{}",ast);
    println!("Executing program");
    ast.eval(&mut bigstep::Store::new())
	.map_err(|err| println!("Evaluation error: {}", err))
}
