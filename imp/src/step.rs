use crate::{error::Error, store::Store, syntax::{Aexpr, Bexpr, Cmd}};
use std::mem;

impl Aexpr {
    fn step(&mut self, s : &Store) -> Result<bool,Error> {
	use Aexpr::*;
	match self {
	    Int (_) => Ok (false),
	    Var (x) => s.get(&x)
		.ok_or(Error::UnboundVariable(String::from(x.clone())))
		.map(|z| { *self = Int(z); true }),
	    Op (o, box Int (z1), box Int (z2)) => {
		*self = Int (o.eval(*z1,*z2)); Ok (true)
	    }
	    Op (_, box Int(_), e2) => e2.step(s),
	    Op (_, e1, _) => e1.step(s)
	}
    }
}

impl Bexpr {
    fn step(&mut self, s : &Store) -> Result<bool,Error> {
	use Bexpr::*;
	match self {
	    Bool (_) => Ok (false),
	    Cop(o, box Aexpr::Int(z1), box Aexpr::Int(z2)) => {
		*self = Bool (o.eval(*z1,*z2)); Ok (true)
	    }
	    Cop (_, box Aexpr::Int (_), e2) => e2.step(s),
	    Cop (_, e1, _) => e1.step(s),
	    Bop (o, box Bool(b1), box Bool(b2)) => {
		*self = Bool (o.eval(*b1,*b2)); Ok (true)
	    }
	    Bop (_, box Bool (_), e2) => e2.step(s),
	    Bop (_, e1, _) => e1.step(s)
	}
    }
}

impl Cmd {
    fn step(&mut self, s : &mut Store) -> Result<bool,Error> {
	use Cmd::*;
	match mem::replace(self,Skip) {
	    Skip => Ok (false),
	    Ass (x, box Aexpr::Int (z)) => {
		s.insert(&x,z); Ok (true) }
	    Ass (x, mut e) =>
		e.step(s)
		.map(|b| {*self = Ass (x,e); b}),
	    Print (box Aexpr::Int (z)) => {
		println!("{}",z); Ok (true) }
	    Print (mut e) =>
		e.step(s)
		.map(|b| {*self = Print (e); b}),
	    Seq (box Skip, box c2) => { *self = c2; Ok (true) }
	    Seq (mut c1, c2) =>
		c1.step(s)
		.map(|b| {*self = Seq (c1,c2); b}),
	    If (box Bexpr::Bool(true), box c1, _)  => { *self = c1; Ok (true) }
	    If (box Bexpr::Bool(false), _, box c2) => { *self = c2; Ok (true) }
	    If (mut e, c1, c2) =>
		e.step(s)
		.map(|b| {*self = If (e,c1,c2); b}),
	    While (box e, box c) => {
		*self = If
		    (box e.clone(),
		     box Seq (box c.clone(),
			      box While (box e.clone(), box c.clone())),
		     box Skip); Ok (true) }
	}
    }
}
