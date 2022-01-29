// Arithmetic operators.
#[derive(Clone,Copy)]
pub enum Aop { AAdd, ASub, AMul }

// Arithmetic expressions.
pub enum Aexpr {
    AInt(i32),
    AVar(String),
    AOp(Aop,Box<Aexpr>,Box<Aexpr>)
}

// Comparison operators.
#[derive(Clone,Copy)]
pub enum Cop { CEq, CLt }

// Boolean operators.
#[derive(Clone,Copy)]
pub enum Bop { BAnd, BOr }

// Boolean expressions.
pub enum Bexpr {
    BBool(bool),
    BCop(Cop,Box<Aexpr>,Box<Aexpr>),
    BBop(Bop,Box<Bexpr>,Box<Bexpr>)
}

// Commands.
pub enum Cmd {
    CSkip,
    CAss(String,Box<Aexpr>),
    CSeq(Box<Cmd>,Box<Cmd>),
    CIf(Box<Bexpr>,Box<Cmd>,Box<Cmd>),
    CWhile(Box<Bexpr>,Box<Cmd>)
}
