use core::fmt;

#[derive(Debug)]
pub struct ExpiredAccessTokenError;

#[derive(Debug)]
pub struct ExpiredRefreshTokenError;

#[derive(Debug)]
pub struct TokenValidationNotSuccessfull;

impl fmt::Display for ExpiredAccessTokenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Access token has expired!")
    }
}

impl fmt::Display for ExpiredRefreshTokenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Refresh token has expired")
    }
}

impl fmt::Display for TokenValidationNotSuccessfull {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Validation of the access token was not successfull!")
    }
}

impl std::error::Error for ExpiredAccessTokenError {}
impl std::error::Error for ExpiredRefreshTokenError {}
impl std::error::Error for TokenValidationNotSuccessfull {}
