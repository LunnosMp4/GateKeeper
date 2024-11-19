use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error};
use actix_web::dev::{Transform, Service};
use futures::future::{ok, Ready};
use futures::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use crate::utils::jwt::validate_jwt;

pub struct JwtValidator;

impl<S, B> Transform<S, ServiceRequest> for JwtValidator
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = JwtValidatorMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(JwtValidatorMiddleware { service })
    }
}

pub struct JwtValidatorMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for JwtValidatorMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let token = req.headers().get("Authorization").and_then(|v| v.to_str().ok());

        if let Some(token) = token {
            if validate_jwt(token) {
                let fut = self.service.call(req);
                return Box::pin(async move {
                    let res = fut.await?;
                    Ok(res)
                });
            }
        }

        Box::pin(async { Err(actix_web::error::ErrorUnauthorized("Invalid JWT token")) })
    }
}