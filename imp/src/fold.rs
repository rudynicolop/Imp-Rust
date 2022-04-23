use crate::syntax::{Aop, Aexpr /*, Bexpr, Cmd*/};
// use std::mem;

// Constant folding.

impl Aexpr {
    fn fold(&mut self) {
	use Aexpr::*;
	match self {
	    Int (_) |
	    Var (_) => (),
	    Op (o, e1, e2) => {
		e1.fold();
		e2.fold();
		use Aop::*;
		match (o.clone(), e1, e2) {
		    (_, box Int (z1), box Int (z2)) => *self = Int(o.eval(*z1,*z2)),
		    (Add, box Int (0), box e) |
		    (Add, box e, box Int (0)) |
		    (Mul, box Int (1), box e) |
		    (Mul, box e, box Int (1)) |
		    (Sub, box e, box Int (0)) => *self = *e,
		    (_, _, _) => ()
		}
	    }
	}
    }
}
