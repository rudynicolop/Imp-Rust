use crate::syntax::{Aexpr, Aop, Bexpr, Bop, Cmd, Cop};
use std::collections::HashMap;

#[derive(Default)]
struct Store(HashMap<String, i32>);

impl Store {
    fn get(&self, var: &str) -> Result<i32, String> {
        self.0
            .get(var)
            .copied()
            .ok_or_else(|| String::from("Unbound variable"))
    }

    fn insert(&mut self, var: &str, value: i32) {
        self.0.insert(String::from(var), value);
    }
}

impl Aop {
    fn eval(&self, z1: i32, z2: i32) -> i32 {
        match self {
            Aop::Add => z1 + z2,
            Aop::Sub => z1 - z2,
            Aop::Mul => z1 * z2,
        }
    }
}

impl Cop {
    fn eval(&self, z1: i32, z2: i32) -> bool {
        match self {
            Cop::Eq => z1 == z2,
            Cop::Lt => z1 < z2,
        }
    }
}

impl Bop {
    fn eval(&self, b1: bool, b2: bool) -> bool {
        match self {
            Bop::And => b1 && b2,
            Bop::Or => b1 || b2,
        }
    }
}

impl Aexpr {
    fn eval(&self, s: &Store) -> Result<i32, String> {
        match self {
            Aexpr::Int(z) => Ok(*z),
            Aexpr::Var(x) => s.get(x),
            Aexpr::Op(o, e1, e2) => {
                let z1 = e1.eval(s)?;
                let z2 = e2.eval(s)?;
                Ok(o.eval(z1, z2))
            }
        }
    }
}

impl Bexpr {
    fn eval(&self, s: &Store) -> Result<bool, String> {
        match self {
            Bexpr::Bool(b) => Ok(*b),
            Bexpr::Cop(o, e1, e2) => {
                let z1 = e1.eval(s)?;
                let z2 = e2.eval(s)?;
                Ok(o.eval(z1, z2))
            }
            Bexpr::Bop(o, e1, e2) => {
                let b1 = e1.eval(s)?;
                let b2 = e2.eval(s)?;
                Ok(o.eval(b1, b2))
            }
        }
    }
}

impl Cmd {
    fn eval(&self, s: &mut Store) -> Result<(), String> {
        match self {
            Cmd::Skip => Ok(()),
            Cmd::Ass(x, e) => {
                let z = e.eval(s)?;
                Ok(s.insert(x, z))
            }
            Cmd::Seq(c1, c2) => {
                let _ = c1.eval(s)?;
                c2.eval(s)
            }
            Cmd::If(e, c1, c2) => {
                let b = e.eval(s)?;
                if b {
                    c1.eval(s)
                } else {
                    c2.eval(s)
                }
            }
            Cmd::While(e, w) => {
                let b = e.eval(s)?;
                if b {
                    let _ = w.eval(s)?;
                    self.eval(s)
                } else {
                    Ok(())
                }
            }
        }
    }
}
