use std::pin::Pin;
use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures::future::{ok, Ready};
use std::task::{Context, Poll};
use time::PrimitiveDateTime;

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
        let db_pool = self.db_pool.clone();

        Box::pin(async move {
            let res = fut.await?;
            let request = res.request();
            let api_key = request
                .headers()
                .get("x-api-key")
                .and_then(|v| v.to_str().ok())
                .unwrap();
            let path = request.path();
            let method = request.method().as_str();
            let now = time::OffsetDateTime::now_utc();
            let primitive_now = PrimitiveDateTime::new(now.date(), now.time());
            let binding = request.connection_info().clone();
            let peer_addr = binding.peer_addr().unwrap();
            let status_code = res.status().as_u16() as i32;
            let user_id = sqlx::query!("SELECT id FROM users WHERE api_key = $1", api_key)
                .fetch_one(&db_pool)
                .await
                .map(|r| r.id)
                .unwrap();

            let _ = sqlx::query!(
                r#"
                INSERT INTO api_usage (user_id, api_key, request_path, request_method, request_time, request_ip, status_code)
                VALUES ($1, $2, $3, $4, $5, $6, $7)
                "#,
                user_id,
                api_key,
                path,
                method,
                primitive_now,
                peer_addr,
                status_code
            )
            .execute(&db_pool)
            .await;

            Ok(res)
        })
    }
}