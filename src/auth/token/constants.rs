use jsonwebtoken::Algorithm;

pub const ACCESS_TOKEN_EXPIRATION: i64 = 60 * 60;
pub const ALGORITHM: Algorithm = Algorithm::HS256;
pub const REFRESH_ALGORITHM: Algorithm = Algorithm::HS512;
pub const REFRESH_KEY_LENGTH: usize = 64;
pub const REFRESH_TOKEN_EXPIRATION: i64 = 60 * 60 * 24;
pub const SECRET_KEY_LENGTH: usize = 64;
