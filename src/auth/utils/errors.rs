use core::fmt;

#[derive(Debug)]
pub struct HashPasswordError;

impl fmt::Display for HashPasswordError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Something went wrong during password encoding!")
    }
}

impl std::error::Error for HashPasswordError {}
