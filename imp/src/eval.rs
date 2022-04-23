use crate::{error::Error, store::Store, syntax::{Aexpr, Bexpr, Cmd}};

impl Aexpr {
    fn eval(&self, s: &Store) -> Result<i32, Error> {
        match self {
            Aexpr::Int(z) => Ok(*z),
            Aexpr::Var(x) =>
		s.get(x)
		.ok_or(Error::UnboundVariable(String::from(x))),
            Aexpr::Op(o, e1, e2) => {
                let z1 = e1.eval(s)?;
                let z2 = e2.eval(s)?;
                Ok(o.eval(z1, z2))
            }
        }
    }
}

impl Bexpr {
    fn eval(&self, s: &Store) -> Result<bool, Error> {
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
    pub fn eval(&self, s: &mut Store) -> Result<(), Error> {
        match self {
            Cmd::Skip => Ok(()),
            Cmd::Ass(x, e) => {
                let z = e.eval(s)?;
                Ok(s.insert(x, z))
            }
	    Cmd::Print(e) => {
		let z = e.eval(s)?;
		println!("OUTPUT: {}",z);
		Ok(())
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
