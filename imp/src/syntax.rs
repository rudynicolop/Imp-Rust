// Arithmetic operAtors.
pub enum Aop { AAdd, ASub, AMul }

// Arithmetic expressions.
pub enum Aexpr {
    AInt(i32),
    AVar(String),
    AOp(Aop,Box<Aexpr>,Box<Aexpr>)
}

// CompArison operAtors.
pub enum Cop { CEq, CLt }

// BooleAn operAtors.
pub enum Bop { BAnd, BOr }

// BooleAn expressions.
pub enum Bexpr {
    BBool(bool),
    BCop(Cop,Box<Aexpr>,Box<Aexpr>),
    BBop(Bop,Box<Bexpr>,Box<Bexpr>)
}

// CommAnds.
pub enum Cmd {
    CSkip,
    CAss(String,Box<Aexpr>),
    CSeq(Box<Cmd>,Box<Cmd>),
    CIf(Box<Bexpr>,Box<Cmd>,Box<Cmd>),
    CWhile(Box<Bexpr>,Box<Cmd>)
}
