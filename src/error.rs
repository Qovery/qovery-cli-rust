pub type QResult<T> = Result<T, Error>;

pub enum Error {
    AuthTokenNotFound,
    AuthTokenExpired,
    AccessTokenFileDoesNotExist,
    Unknown(String),
}
