#![feature(box_syntax)]
#![feature(box_patterns)]
#![feature(box_into_inner)]

extern crate codespan;
extern crate clap;
extern crate lalrpop_util;
extern crate peeking_take_while;

pub mod error;
pub mod eval;
pub mod lexer;
pub mod op;
pub mod parser;
pub mod step;
pub mod store;
pub mod syntax;
