use std::path::PathBuf;
use imp::{lexer,parser,store,syntax};
use codespan::CodeMap;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(name="imp")]
struct Args {
    #[clap(short, long)]
    eval: bool, // evaluate
    
    #[clap(short, long)]
    step: bool, // small-step
    
    #[clap(parse(from_os_str))]
    path: PathBuf, // file path
}

fn main() -> Result<(),()> {
    // Parse command-line arguments.
    let args : Args = Args::parse();

    // Create filemap.
    let mut codemap = CodeMap::new();
    let file = codemap.add_filemap_from_disk(args.path)
	.map_err(|err| println!("File error: {}",err))?;

    // Lex.
    let tokens = lexer::tokenize(&file)
	.map_err(|err|
		 println!(
		     "Lexical error at line {} column {}: {}",
		     err.0,err.1,err.2))?;

    // Parse.
    let mut ast: syntax::Cmd = parser::parse(&file,tokens)?;
    
    println!("------------ Program parsed as ------------");
    println!("{}",ast);
    if args.step {
	println!("------------ Stepping program ------------");
	return ast.normalize(&mut store::Store::new())
	    .map_err(|err| println!("Evaluation error: {}", err));
    }
    if args.eval {
	println!("------------ Executing program ------------");
	return ast.eval(&mut store::Store::new())
	    .map_err(|err| println!("Evaluation error: {}", err));
    }
    Ok (())
}
