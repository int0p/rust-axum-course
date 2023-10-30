use axum::{response::{IntoResponse, Response}, http::StatusCode};

pub type Result<T> = core::result::Result<T,Error>;
#[derive(Debug)]
pub enum Error{
    LoginFail,

    // -- Model errors
    TicketDeleteFailIdNotFound {id: u64},
}

// 주의! server error를 client에게 보여주지 않는다. 
impl IntoResponse for Error{
    fn into_response(self)->Response{
        println!("->> {:<12} - {self:?}","INTO_RESPONSE");

        (StatusCode::INTERNAL_SERVER_ERROR,"UNHANDLED_CLIENT_ERROR").into_response()
    }
}

