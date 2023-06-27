use axum::{response::{IntoResponse, Response}, http::StatusCode};
use serde::Serialize;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Clone, Debug, Serialize, strum_macros::AsRefStr)]
#[serde(tag="type", content="data")]
pub enum Error {
    LoginFailed,

    // region: --- Auth Errors.
    AuthFailedAuthTokenCookies,
    AuthFailedTokenWrongFormat,
    AuthFailedCtxNotInrequestExt,
    // endregion: --- Auth Errors.

    // region: --- Model Error
    TicketDeleteNotFound {id: u64},
    TicketUpdateNotFound {id: u64},
    TicketGetNotFound {id: u64}
    // endregion: --- Model Error
}

// region: --- impl Error
impl IntoResponse for Error {
    // type Body = String;
    // type BodyError = <Self::Body as IntoResponse>::BodyError;

    fn into_response(self) -> Response {
        println!("->> {:<12} - {self:?}", "INTO_RESPONSE");

        // Create a placeholdder for axum respon.
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        // Insert the Error into the response.
        response.extensions_mut().insert(self);

        response
    }
}


impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> core::result::Result<(), std::fmt::Error> {
        
        write!(fmt, "{self:?}")
    }
}
// endregion: --- impl Error


// region: --- Client Error
impl Error {
    pub fn client_satus_and_error(&self) -> (StatusCode, ClientError){
        #[allow(unreachable_patterns)]
        match self {
            // -- Login Error
            Self::LoginFailed => (StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL),

            // -- Auth Errors
            Self::AuthFailedAuthTokenCookies
            | Self::AuthFailedTokenWrongFormat
            | Self::AuthFailedCtxNotInrequestExt => {
                (StatusCode::UNAUTHORIZED, ClientError::NO_AUTH)
            }

            // -- Model Error
            Self::TicketDeleteNotFound {..} => {
                (StatusCode::BAD_REQUEST, ClientError::INVALID_PARAMS)
            }

            // -- Fallback
            _ => (StatusCode::INTERNAL_SERVER_ERROR, ClientError::SERVCE_ERROR),
        }
    }
}

#[derive(Debug, strum_macros::AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ClientError {
    LOGIN_FAIL,
    NO_AUTH,
    INVALID_PARAMS,
    SERVCE_ERROR,
}
// endregion: --- Client Error

