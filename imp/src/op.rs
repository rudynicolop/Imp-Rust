use crate::syntax::{Aop, Cop, Bop};

impl Aop {
    pub fn eval(&self, z1: i32, z2: i32) -> i32 {
	use Aop::*;
        match self {
            Add => z1 + z2,
            Sub => z1 - z2,
            Mul => z1 * z2,
        }
    }
}

impl Cop {
    pub fn eval(&self, z1: i32, z2: i32) -> bool {
	use Cop::*;
        match self {
            Eq => z1 == z2,
            Lt => z1 < z2,
        }
    }
}

impl Bop {
    pub fn eval(&self, b1: bool, b2: bool) -> bool {
	use Bop::*;
        match self {
            And => b1 && b2,
            Or  => b1 || b2,
        }
    }
}
