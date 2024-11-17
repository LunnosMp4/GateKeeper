use std::pin::Pin;
use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures::future::{ok, Ready};
use std::task::{Context, Poll};

pub struct ApiUsageLogger {
    db_pool: sqlx::PgPool,
}

impl ApiUsageLogger {
    pub fn new(db_pool: sqlx::PgPool) -> Self {
        Self { db_pool }
    }
}

impl<S, B> Transform<S, ServiceRequest> for ApiUsageLogger
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = ApiUsageLoggerMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(ApiUsageLoggerMiddleware {
            service,
            db_pool: self.db_pool.clone()
        })
    }
}

pub struct ApiUsageLoggerMiddleware<S> {
    service: S,
    db_pool: sqlx::PgPool,
}

impl<S, B> Service<ServiceRequest> for ApiUsageLoggerMiddleware<S>
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
        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            println!("TODO: Log API usage to database");
            Ok(res)
        })
    }
}