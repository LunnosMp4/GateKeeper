use actix_web::HttpResponse;
use std::pin::Pin;
use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures::future::{ok, Ready};
use std::task::{Context, Poll};
use sqlx::PgPool;

pub struct AdminValidator {
    pub db_pool: PgPool,
}

impl AdminValidator {
    pub fn new(db_pool: PgPool) -> Self {
        Self { db_pool }
    }
}

impl<S, B> Transform<S, ServiceRequest> for AdminValidator
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AdminValidatorMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AdminValidatorMiddleware {
            service,
            db_pool: self.db_pool.clone(),
        })
    }
}

pub struct AdminValidatorMiddleware<S> {
    service: S,
    db_pool: PgPool,
}

impl<S, B> Service<ServiceRequest> for AdminValidatorMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
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
            .map(String::from);

        let fut = self.service.call(req);

        Box::pin(async move {
            if let Some(api_key) = api_key {
                let user = sqlx::query!(
                "SELECT permission FROM users WHERE api_key = $1",
                api_key
            )
                    .fetch_optional(&db_pool)
                    .await;

                if let Ok(Some(user)) = user {
                    if user.permission == 1 {
                        return fut.await;
                    } else {
                        return Err(actix_web::error::ErrorUnauthorized(
                            "You are not authorized to access this resource",
                        ));
                    }
                } else {
                    return Err(actix_web::error::ErrorUnauthorized("Invalid or missing API key"));
                }
            } else {
                return Err(actix_web::error::ErrorUnauthorized("Invalid or missing API key"));
            }
        })
    }
}
