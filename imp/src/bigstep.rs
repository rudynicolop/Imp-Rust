use std::result::Result;
use std::collections::HashMap;
use crate::syntax::{Aexpr,Aop};
use do_notation::m;

type Sigma = HashMap<String,i32>;

fn deref_option <T:Copy> (o: Option<&T>) -> Option<T> {
    m! { t <- o; Option::Some(*t) }
}

fn aop_eval (o : Aop, z1 : i32, z2 : i32) -> i32 {
    match o {
	Aop::AAdd => z1 + z2,
	Aop::ASub => z1 - z2,
	Aop::AMul => z1 * z2
    }
}

fn aeval (s : &Sigma, e : Aexpr) -> Result<i32,String> {
    match e {
	Aexpr::AInt(z) => Result::Ok(z),
	Aexpr::AVar(x) =>
	    deref_option(s.get(&x)).ok_or("Unbound variable".into()),
	Aexpr::AOp(o,e1,e2) => m! {
	    z1 <- aeval(s,*e1);
	    z2 <- aeval(s,*e2);
	    Result::Ok(aop_eval(o,z1,z2))
	}
    }
}
