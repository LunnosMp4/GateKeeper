use std::pin::Pin;
use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures::future::{ok, Ready};
use std::task::{Context, Poll};

pub struct ApiKeyValidator {
    db_pool: sqlx::PgPool,
}

impl ApiKeyValidator {
    pub fn new(db_pool: sqlx::PgPool) -> Self {
        Self { db_pool }
    }
}

impl<S, B> Transform<S, ServiceRequest> for ApiKeyValidator
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = ApiKeyValidatorMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(ApiKeyValidatorMiddleware {
            service,
            db_pool: self.db_pool.clone()
        })
    }
}

pub struct ApiKeyValidatorMiddleware<S> {
    service: S,
    db_pool: sqlx::PgPool,
}

impl<S, B> Service<ServiceRequest> for ApiKeyValidatorMiddleware<S>
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
        let db_pool = self.db_pool.clone();

        let api_key = req
            .headers()
            .get("x-api-key")
            .and_then(|v| v.to_str().ok())
            .map(|v| v.to_string());

        let fut = self.service.call(req);

        Box::pin(async move {
            if let Some(api_key) = api_key {
                if validate_api_key(&db_pool, &api_key).await {
                    return fut.await
                } else {
                    Err(actix_web::error::ErrorUnauthorized("Invalid or missing API key"))
                }
            } else {
                Err(actix_web::error::ErrorUnauthorized("Invalid or missing API key"))
            }
        })
    }
}

async fn validate_api_key(db_pool: &sqlx::PgPool, api_key: &str) -> bool {
    let query = "SELECT EXISTS(SELECT 1 FROM users WHERE api_key = $1)";
    sqlx::query_scalar(query)
        .bind(api_key)
        .fetch_one(db_pool)
        .await.unwrap_or_else(|_| false)
}