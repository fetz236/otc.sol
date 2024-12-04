use actix_service::{Service, Transform};
use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    Error, HttpMessage,
};
use actix_web_httpauth::extractors::AuthExtractor;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use futures_util::future::{ok, LocalBoxFuture, Ready};
use std::sync::Arc;
use std::task::{Context, Poll};
use crate::auth::verify_token;

pub struct AuthMiddleware;

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthMiddlewareMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddlewareMiddleware {
            service: Arc::new(service),
        })
    }
}

pub struct AuthMiddlewareMiddleware<S> {
    service: Arc<S>,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        // Use the service wrapped in Arc
        self.service.poll_ready(cx)
    }

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        // Clone the Arc to move into the async block
        let service = self.service.clone();

        Box::pin(async move {
            // Extract BearerAuth asynchronously
            if let Ok(credentials) = BearerAuth::from_service_request(&req).await {
                // Validate token asynchronously
                if let Ok(_) = auth_validator(&mut req, credentials).await {
                    // Token is valid; proceed with the request
                    return service.call(req).await;
                }
            }

            // Token is invalid or not provided; respond with an error
            let err = actix_web::error::ErrorUnauthorized("Invalid or missing token");
            Err(err)
        })
    }
}

pub async fn auth_validator(
    req: &mut ServiceRequest,
    credentials: BearerAuth,
) -> Result<(), actix_web::Error> {
    let token = credentials.token();

    match verify_token(token) {
        Ok(claims) => {
            req.extensions_mut().insert(claims);
            Ok(())
        }
        Err(err) => Err(actix_web::error::ErrorUnauthorized(err)),
    }
}
