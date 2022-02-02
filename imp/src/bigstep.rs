use crate::syntax::{Aexpr, Aop, Bexpr, Bop, Cmd, Cop};
use do_notation::m;
use std::collections::HashMap;
use std::result::Result;

type Store = HashMap<String, i32>;

fn aop_eval(o: Aop, z1: i32, z2: i32) -> i32 {
    match o {
        Aop::Add => z1 + z2,
        Aop::Sub => z1 - z2,
        Aop::Mul => z1 * z2,
    }
}

fn cop_eval(o: Cop, z1: i32, z2: i32) -> bool {
    match o {
        Cop::Eq => z1 == z2,
        Cop::Lt => z1 < z2,
    }
}

fn bop_eval(o: Bop, b1: bool, b2: bool) -> bool {
    match o {
        Bop::And => b1 && b2,
        Bop::Or => b1 || b2,
    }
}

fn aeval(s: &Store, e: &Aexpr) -> Result<i32, String> {
    match e {
        Aexpr::Int(z) => Result::Ok(*z),
        Aexpr::Var(x) => s.get(x).copied().ok_or("Unbound variable".into()),
        Aexpr::Op(o, e1, e2) => m! {
            z1 <- aeval(s,e1);
            z2 <- aeval(s,e2);
            Result::Ok(aop_eval(*o,z1,z2))
        },
    }
}

fn beval(s: &Store, e: &Bexpr) -> Result<bool, String> {
    match e {
        Bexpr::Bool(b) => Result::Ok(*b),
        Bexpr::Cop(o, e1, e2) => m! {
            z1 <- aeval(s,e1);
            z2 <- aeval(s,e2);
            Result::Ok(cop_eval(*o,z1,z2))
        },
        Bexpr::Bop(o, e1, e2) => m! {
            b1 <- beval(s,e1);
            b2 <- beval(s,e2);
            Result::Ok(bop_eval(*o,b1,b2))
        },
    }
}

fn eval(s: &mut Store, c: &Cmd) -> Result<(), String> {
    match c {
        Cmd::Skip => Result::Ok(()),
        Cmd::Ass(x, e) => m! {
            z <- aeval(s,e);
            Result::Ok(drop(s.insert(x.clone(),z)))
        },
        Cmd::Seq(c1, c2) => m! {
            _ <- eval(s,c1); eval(s,c2)
        },
        Cmd::If(e, c1, c2) => m! {
            b <- beval(s,e);
            if b { eval(s,c1) } else { eval(s,c2) }
        },
        Cmd::While(e, w) => m! {
            b <- beval(s,e);
            if b {
            m! { _ <- eval(s,w); eval(s,c) }
            } else {
            Result::Ok(())
            }
        },
    }
}
