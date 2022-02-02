use crate::syntax::{Aexpr, Aop, Bexpr, Bop, Cmd, Cop};
use do_notation::m;
use std::collections::HashMap;
use std::result::Result;

type Store = HashMap<String, i32>;

fn deref_option<T: Copy>(o: Option<&T>) -> Option<T> {
    m! { t <- o; Option::Some(*t) }
}

fn ignore<A>(_: A) -> () {
    ()
}

fn aop_eval(o: Aop, z1: i32, z2: i32) -> i32 {
    match o {
        Aop::AAdd => z1 + z2,
        Aop::ASub => z1 - z2,
        Aop::AMul => z1 * z2,
    }
}

fn cop_eval(o: Cop, z1: i32, z2: i32) -> bool {
    match o {
        Cop::CEq => z1 == z2,
        Cop::CLt => z1 < z2,
    }
}

fn bop_eval(o: Bop, b1: bool, b2: bool) -> bool {
    match o {
        Bop::BAnd => b1 && b2,
        Bop::BOr => b1 || b2,
    }
}

fn aeval(s: &Store, e: &Aexpr) -> Result<i32, String> {
    match e {
        Aexpr::AInt(z) => Result::Ok(*z),
        Aexpr::AVar(x) => deref_option(s.get(x)).ok_or("Unbound variable".into()),
        Aexpr::AOp(o, e1, e2) => m! {
            z1 <- aeval(s,e1);
            z2 <- aeval(s,e2);
            Result::Ok(aop_eval(*o,z1,z2))
        },
    }
}

fn beval(s: &Store, e: &Bexpr) -> Result<bool, String> {
    match e {
        Bexpr::BBool(b) => Result::Ok(*b),
        Bexpr::BCop(o, e1, e2) => m! {
            z1 <- aeval(s,e1);
            z2 <- aeval(s,e2);
            Result::Ok(cop_eval(*o,z1,z2))
        },
        Bexpr::BBop(o, e1, e2) => m! {
            b1 <- beval(s,e1);
            b2 <- beval(s,e2);
            Result::Ok(bop_eval(*o,b1,b2))
        },
    }
}

fn eval(s: &mut Store, c: &Cmd) -> Result<(), String> {
    match c {
        Cmd::CSkip => Result::Ok(()),
        Cmd::CAss(x, e) => m! {
            z <- aeval(s,e);
            Result::Ok(ignore(s.insert(x.clone(),z)))
        },
        Cmd::CSeq(c1, c2) => m! {
            _ <- eval(s,c1); eval(s,c2)
        },
        Cmd::CIf(e, c1, c2) => m! {
            b <- beval(s,e);
            if b { eval(s,c1) } else { eval(s,c2) }
        },
        Cmd::CWhile(e, w) => m! {
            b <- beval(s,e);
            if b {
            m! { _ <- eval(s,w); eval(s,c) }
            } else {
            Result::Ok(())
            }
        },
    }
}
