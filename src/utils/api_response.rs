use std::{
    fmt::{Debug, Display},
    marker::PhantomData,
    task::Poll,
};

use actix_web::{
    body::{self, BodySize, BoxBody, MessageBody},
    web::{self, Bytes},
    HttpResponse, Responder, ResponseError,
};
use serde::Serialize;

pub trait UniformData: Debug + Clone + Serialize + Send + Sync {}
impl<T: Debug + Clone + Serialize + Send + Sync> UniformData for T {}

#[derive(Debug, Serialize, Clone)]
pub struct ApiResponseMessage<T: UniformData> {
    code: i8,
    msg: String,
    data: T,
    _marker: PhantomData<T>,
}

// impl<T: UniformData> MessageBody for ApiResponseMessage<T> {
//     type Error = String;
//     fn size(&self) -> actix_web::body::BodySize {
//         match serde_json::to_string(&self) {
//             Ok(serialized) => BodySize::Sized(serialized.len() as u64),
//             Err(_) => BodySize::None, // Handle error properly in a real implementation
//         }
//     }

//     fn poll_next(
//         self: std::pin::Pin<&mut Self>,
//         cx: &mut std::task::Context<'_>,
//     ) -> std::task::Poll<Option<Result<actix_web::web::Bytes, Self::Error>>> {

//         match serde_json::to_string(&self.get_data()) {
//             Ok(payload_string) => {
//                 let payload_bytes = Bytes::from(payload_string);
//                 Poll::Ready(Some(Ok(payload_bytes)))
//             }
//             Err(_) => Poll::Ready(Some(Err("".to_string()))), // Handle serialization error
//         }

//     }
// }
impl<T: UniformData> ApiResponseMessage<T> {
    pub fn new(code: i8, msg: String, data: T) -> Self {
        ApiResponseMessage {
            code,
            msg,
            data,
            _marker: PhantomData::default(),
        }
    }

    pub fn success_with_data(data: T) -> Self {
        let msg = String::default();
        let code = i8::default();
        ApiResponseMessage {
            code,
            msg,
            data,
            _marker: PhantomData::default(),
        }
    }
    pub fn error_with_data(data: T) -> Self {
        let code = -1;
        let msg = String::default();
        ApiResponseMessage {
            code,
            msg,
            data,
            _marker: PhantomData::default(),
        }
    }
    pub fn customize_error_msg(mut self, msg: String) -> Self {
        self.msg = msg;
        self
    }

    pub fn customize_error_code(mut self, code: i8) -> Self {
        self.code = code;
        self
    }
    pub fn reverse(mut self) -> Self {
        self.code = -self.code;
        self
    }
    pub fn get_data(self) -> T {
        self.data
    }
}

impl<T: UniformData + Default> ApiResponseMessage<T> {
    fn default() -> Self {
        ApiResponseMessage {
            code: 0,
            msg: String::default(),
            data: T::default(),
            _marker: PhantomData::default(),
        }
    }
}

impl<T: UniformData + Default> ApiResponseMessage<T> {
    pub fn error() -> Self {
        let msg = String::default();
        let data = T::default();
        let _marker = PhantomData::default();
        ApiResponseMessage {
            code: -1,
            msg,
            data,
            _marker,
        }
    }

    pub fn success() -> Self {
        let msg = String::default();
        let data = T::default();
        let _marker = PhantomData::default();
        ApiResponseMessage {
            code: 0,
            msg,
            data,
            _marker,
        }
    }
}
impl<T: UniformData + Default, E: std::error::Error> From<Result<T, E>> for ApiResponseMessage<T> {
    fn from(result: Result<T, E>) -> Self {
        match result {
            Ok(d) => ApiResponseMessage::success_with_data(d),
            Err(e) => {
                let message = format!(
                    "{} ({})",
                    e,
                    e.source().map(|e| e.to_string()).unwrap_or_default()
                );
                ApiResponseMessage::error().customize_error_msg(message)
            }
        }
    }
}

// impl<T: UniformData> From<HttpResponse> for ApiResponseMessage<T> {
//     fn from(response: HttpResponse) -> Self {
//         let status = response.status().as_u16();
//         let a  = if status >= 200 && status < 300 {
//             HttpResponse::Ok().body(ApiResponseMessage::success())
//         } else {
//             let message = format!("HTTP status code: {}", status);
//             ApiResponseMessage::error().customize_error_msg(message)
//         }
//     }
// }

impl<T: UniformData> Responder for ApiResponseMessage<T> {
    type Body = BoxBody;
    fn respond_to(self, _: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        let body = BoxBody::new(serde_json::to_string(&self).unwrap());
        if self.code >= 0 {
            HttpResponse::Ok().body(body)
        } else {
            HttpResponse::InternalServerError().body(body)
        }
    }
}

impl<T: UniformData> ResponseError for ApiResponseMessage<T> {
    fn status_code(&self) -> actix_web::http::StatusCode {
        if self.code >= 0 {
            actix_web::http::StatusCode::OK
        } else {
            actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
        }
    }
    fn error_response(&self) -> HttpResponse<BoxBody> {
        let body = BoxBody::new(serde_json::to_string(&self).unwrap());
        if self.code >= 0 {
            HttpResponse::Ok().body(body)
        } else {
            HttpResponse::InternalServerError().body(body)
        }
    }
}

impl<T: UniformData> Display for ApiResponseMessage<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl<T: UniformData + Default> From<actix_web::Error> for ApiResponseMessage<T> {
    fn from(error: actix_web::Error) -> Self {
        let message = format!("{}", error);
        ApiResponseMessage::error().customize_error_msg(message)
    }
}
