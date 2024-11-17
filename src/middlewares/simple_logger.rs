use std::pin::Pin;
use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures::future::{ok, Ready};
use std::task::{Context, Poll};

pub struct SimpleLogger;

impl<S, B> Transform<S, ServiceRequest> for SimpleLogger
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = SimpleLoggerMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(SimpleLoggerMiddleware { service })
    }
}

pub struct SimpleLoggerMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for SimpleLoggerMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        println!("Incoming request: {:?}", req.path()); // Log the request path

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            println!("Response status: {}", res.status());
            Ok(res)
        })
    }
}
