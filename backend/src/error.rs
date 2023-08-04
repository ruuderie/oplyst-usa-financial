use axum::{response::IntoResponse, http::StatusCode};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub enum Error{
    NotFound,
    PermissionDenied,
    AuthFailNoAuthTokenCookie,
    AuthFailTokenWrongFormat,
    Unknown,
    LoginFailed,
    TicketDeleteFailIdNotFound{id: u32},
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        println!("--> {:<12} - Error - {self:?}", "INTO RES");
        (StatusCode::INTERNAL_SERVER_ERROR,"UNHANDLED_CLIENT_ERROR").into_response()
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> core::result::Result<(), std::fmt::Error> {
        write!(f, "Error: {:?}", self)
    }
}

impl std::error::Error for Error {}