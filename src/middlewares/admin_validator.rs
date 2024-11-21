use std::pin::Pin;
use actix_web::{dev::{Service, ServiceRequest, ServiceResponse, Transform}, Error, HttpMessage};
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

        let user_id = req
            .extensions()
            .get::<String>()
            .cloned()
            .and_then(|id| id.parse::<i32>().ok());

        let fut = self.service.call(req);

        Box::pin(async move {
            let user_id = match user_id {
                Some(id) => id,
                None => return Err(actix_web::error::ErrorUnauthorized("User ID not found")),
            };

            let permission = sqlx::query!("SELECT permission FROM users WHERE id = $1", user_id)
                .fetch_one(&db_pool)
                .await;

            match permission {
                Ok(user) => {
                    if user.permission == 1 {
                        fut.await
                    } else {
                        Err(actix_web::error::ErrorUnauthorized(
                            "You do not have permission to access this resource",
                        ))
                    }
                }
                Err(_) => Err(actix_web::error::ErrorUnauthorized(
                    "You do not have permission to access this resource",
                )),
            }
        })
    }

}
