// Arithmetic operators.
#[derive(Clone, Copy)]
pub enum Aop {
    Add,
    Sub,
    Mul,
}

// Arithmetic expressions.
pub enum Aexpr {
    Int(i32),
    Var(String),
    Op(Aop, Box<Aexpr>, Box<Aexpr>),
}

// Comparison operators.
#[derive(Clone, Copy)]
pub enum Cop {
    Eq,
    Lt,
}

// Boolean operators.
#[derive(Clone, Copy)]
pub enum Bop {
    And,
    Or,
}

// Boolean expressions.
pub enum Bexpr {
    Bool(bool),
    Cop(Cop, Box<Aexpr>, Box<Aexpr>),
    Bop(Bop, Box<Bexpr>, Box<Bexpr>),
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
