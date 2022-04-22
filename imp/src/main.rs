use std::{env, path::{Path, PathBuf}};
use imp::{lexer,parser,bigstep,syntax};
use codespan{CodeMap, FileMap}

fn main() -> Result<(),()> {
    // Parse command-line arguments.
    let args : Vec<String> = env::args().collect();

    // Create path to get file.
    let path = Path::new(&args[1]);
    let buf = PathBuf::new();
    buf.push(path);

    // Create filemap.
    let codemap = CodeMap::new();
    let file = codemap.add_file_map_from_disk(buf)
	.map_err(|err| println!("File error: {}",err))?;

    // Lex.
    let tokens = lexer::tokenize(file)
	.map_err(|err|
		 println!(
		     "Lexical error at line {} column {}: {}",
		     err.0.0,err.0.1,err.1))?;

    // Parse.
    let ast: syntax::Cmd = parser::parse(tokens)
	.map_err(|err| println!("Parsing error: {}",err))?;

    println!("Program parsed as");
    println!("{}",ast);
    println!("Executing program");

    ast.eval(&mut bigstep::Store::new())
	.map_err(|err| println!("Evaluation error: {}", err))
}
