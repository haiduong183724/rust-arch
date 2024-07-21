use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    HttpMessage,
};
use actix_web_lab::middleware::Next;

use crate::utils::{api_response::ApiResponseMessage, jwt::decode_jwt};

pub async fn auth_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, actix_web::Error> {
    let headers = req.headers();
    let auth_header = headers.get("Authorization");
    if let Some(auth_header) = auth_header {
        let auth_header = auth_header.to_str().unwrap();
        if auth_header.starts_with("Bearer") {
            let token = auth_header.split_whitespace().nth(1).unwrap();
            let _claim = decode_jwt(token).unwrap();
            req.extensions_mut().insert(_claim);
            next.call(req).await.map_err(|e| e)
        } else {
            Err(actix_web::Error::from(ApiResponseMessage::error_with_data(
                "Can not parse token1",
            )))
        }
    } else {
        Err(actix_web::Error::from(ApiResponseMessage::error_with_data(
            "Can not parse token2",
        )))
    }
}
