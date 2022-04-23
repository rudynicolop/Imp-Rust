use std::fmt;

pub enum Error {
    UnboundVariable(String)
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	match self {
	    Error::UnboundVariable(x) =>
		write!(f, "Unbound Variable {}", x)
	}
    }
}
