use actix_web::{dev::ServiceRequest, Error};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use futures_util::future::{ready, Ready};
use crate::auth::verify_token;
use actix_web::HttpMessage;
use crate::auth;

pub fn auth_validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Ready<Result<ServiceRequest, (Error, ServiceRequest)>> {
    let token = credentials.token();

    match verify_token(token) {
        Ok(claims) => {
            req.extensions_mut().insert(claims);
            ready(Ok(req))
        }
        Err(err) => ready(Err((actix_web::error::ErrorUnauthorized(err), req))),
    }
}