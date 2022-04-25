use crate::syntax::{Aop, Aexpr, Cop, Bop, Bexpr, Cmd};

// Constant folding.

impl Aexpr {
    fn fold(& self) -> Aexpr {
	use Aexpr::*;
	match self {
	    Int (z) => Int (*z),
	    Var (x) => Var (x.clone()),
	    Op (o, box e1, box e2) => {
		use Aop::*;
		match (*o, e1.fold(), e2.fold()) {
		    (_, Int (z1), Int (z2))
			=> Int(o.eval(z1, z2)),
		    (Add, Int (0),e) |
		    (Add, e, Int (0)) |
		    (Mul, Int (1), e) |
		    (Mul, e, Int (1)) |
		    (Sub, e, Int (0)) => e,
		    (Mul, Int (0), _) |
		    (Mul, _, Int (0)) => Int (0),
		    (Sub, e1, e2)
			=> if e1 == e2 {
			    Int (0)
			} else {
			    Op (Sub, box e1, box e2) },
		    (_, e1, e2) => Op (*o, box e1, box e2)
		}
	    }
	}
    }
}

impl Bexpr {
    fn fold(& self) -> Bexpr {
	use Bexpr::*;
	match self {
	    Bool (b) => Bool (*b),
	    Not (box e) =>
		match e.fold() {
		    Bool (b) => Bool (!b),
		    e => Not (box e)
		},
	    COp (o, box e1, box e2) => {
		use Cop::*;
		use Aexpr::*;
		match (*o, e1.fold(), e2.fold()) {
		    (_, Int(z1), Int(z2)) => Bool (o.eval(z1,z2)),
		    (Eq, e1, e2) =>
			if e1 == e2 {
			    Bool (true)
			} else {
			    COp (Eq, box e1, box e2) },
		    (_, e1, e2) => COp (*o, box e1, box e2)
		}
	    }
	    BOp (o, box e1, box e2) => {
		use Bop::*;
		match (*o, e1.fold(), e2.fold()) {
		    (_, Bool (b1), Bool (b2)) => Bool (o.eval(b1,b2)),
		    (And, Bool (true), e) |
		    (And, e, Bool (true)) |
		    (Or, Bool (false), e) |
		    (Or, e, Bool (false)) => e,
		    (And, e @ Bool (false), _) |
		    (And, _, e @ Bool (false)) |
		    (Or, e @ Bool (true), _) |
		    (Or, _, e @ Bool (true)) => e,
		    (_, e1, e2)
			=> if e1 == e2 { e1 } else { BOp (*o, box e1, box e2) }
		}
	    }
	}
    }
}

impl Cmd {
    pub fn fold (& self) -> Cmd {
	use Cmd::*;
	match self {
	    Skip => Skip,
	    Ass (x, box e) => Ass (x.clone(), box e.fold()),
	    Print (box e) => Print (box e.fold()),
	    Seq (box c1, box c2) =>
		match (c1.fold(), c2.fold()) {
		    (Skip, c) |
		    (c, Skip) => c,
		    (c1, c2) => Seq (box c1, box c2)
		}
	    If (box e, box c1, box c2)
		=> match e.fold() {
		    Bexpr::Bool (b) => if b { c1.fold() } else { c2.fold() },
		    e => if c1 == c2 {
			c1.fold()
		    } else {
			If (box e, box c1.fold(), box c2.fold())
		    }
		},
	    While (box e, box c)
		=> match e.fold() {
		    Bexpr::Bool (false) => Skip,
		    e => While (box e, box c.fold())
		}
	}
    }
}
