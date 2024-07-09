use std::fmt;
use std::error::Error;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum LockServiceErrors {
    ObjectAlreadyLocked,
    ObjectNotLocked,
}

impl fmt::Display for LockServiceErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            LockServiceErrors::ObjectAlreadyLocked => write!(f, "This object has already been locked"),
            LockServiceErrors::ObjectNotLocked => write!(f, "This object was not locked before"),
        }
    }
}

impl Error for LockServiceErrors {}


