use std::fmt;

// Arithmetic operators.
#[derive(Clone, Copy)]
pub enum Aop {
    Add,
    Sub,
    Mul,
}

impl fmt::Display for Aop {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	match self {
	    Aop::Add => write!(f, "+"),
	    Aop::Sub => write!(f, "-"),
	    Aop::Mul => write!(f, "*")
	}
    }
}

// Arithmetic expressions.
pub enum Aexpr {
    Int(i32),
    Var(String),
    Op(Aop, Box<Aexpr>, Box<Aexpr>),
}

impl fmt::Display for Aexpr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	match self {
	    Aexpr::Int(z) => write!(f, "{}", z),
	    Aexpr::Var(x) => write!(f, "{}", x),
	    Aexpr::Op(o,e1,e2) => write!(f, "({} {} {})", e1, o, e2)
	}
    }
}

// Comparison operators.
#[derive(Clone, Copy)]
pub enum Cop {
    Eq,
    Lt,
}

impl fmt::Display for Cop {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	match self {
	    Cop::Eq => write!(f, "=?"),
	    Cop::Lt => write!(f, "<?")
	}
    }
}

// Boolean operators.
#[derive(Clone, Copy)]
pub enum Bop {
    And,
    Or,
}

impl fmt::Display for Bop {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	match self {
	    Bop::And => write!(f, "and"),
	    Bop::Or  => write!(f, "or")
	}
    }
}

// Boolean expressions.
pub enum Bexpr {
    Bool(bool),
    Cop(Cop, Box<Aexpr>, Box<Aexpr>),
    Bop(Bop, Box<Bexpr>, Box<Bexpr>),
}

impl fmt::Display for Bexpr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	match self {
	    Bexpr::Bool(b) => write!(f, "{}", b),
	    Bexpr::Cop(o,e1,e2) => write!(f, "({} {} {})", e1, o, e2),
	    Bexpr::Bop(o,e1,e2) => write!(f, "({} {} {})", e1, o, e2)
	}
    }
}

// Commands.
pub enum Cmd {
    Skip,
    Ass(String, Box<Aexpr>),
    Print(Box<Aexpr>),
    Seq(Box<Cmd>, Box<Cmd>),
    If(Box<Bexpr>, Box<Cmd>, Box<Cmd>),
    While(Box<Bexpr>, Box<Cmd>),
}

impl fmt::Display for Cmd {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	match self {
	    Cmd::Skip => write!(f, "skip"),
	    Cmd::Ass(x,e) => write!(f, "{} := {}", x, e),
	    Cmd::Print(e) => write!(f, "print {}", e),
	    Cmd::Seq(c1,c2) => write!(f, "{};\n{}", c1, c2),
	    Cmd::If(e,c1,c2) =>
		write!(f, "if {} {}\n{}\n{} else {}\n{}\n{}", e, "{", c1, "}", "{", c2, "}"),
	    Cmd::While(e,c) =>
		write!(f, "while {} {}\n{}\n{}", e, "{", c, "}")
	}
    }
}
