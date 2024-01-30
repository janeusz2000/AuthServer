use core::fmt;

#[derive(Debug)]
pub struct NoValueInCookie;

impl fmt::Display for NoValueInCookie {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Value in cookie not found")
    }
}

impl std::error::Error for NoValueInCookie {}
