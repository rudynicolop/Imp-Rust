use std::{env, path::{Path, PathBuf}};
use imp::{lexer,parser,bigstep,syntax};
use codespan::CodeMap;

fn main() -> Result<(),()> {
    // Parse command-line arguments.
    let args : Vec<String> = env::args().collect();

    // Create path to get file.
    let path = Path::new(&args[1]);
    let mut buf = PathBuf::new();
    buf.push(path);

    // Create filemap.
    let mut codemap = CodeMap::new();
    let file = codemap.add_filemap_from_disk(buf)
	.map_err(|err| println!("File error: {}",err))?;

    // Lex.
    let tokens = lexer::tokenize(&file)
	.map_err(|err|
		 println!(
		     "Lexical error at line {} column {}: {}",
		     err.0,err.1,err.2))?;

    // Parse.
    let ast: syntax::Cmd = parser::parse(&file,tokens)?;
    
    println!("Program parsed as");
    println!("{}",ast);
    println!("Executing program");

    ast.eval(&mut bigstep::Store::new())
	.map_err(|err| println!("Evaluation error: {}", err))
}
