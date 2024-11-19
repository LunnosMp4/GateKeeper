use std::pin::Pin;
use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use redis::AsyncCommands;
use futures::future::{ok, Ready};
use std::task::{Context, Poll};
use std::time::Duration;

pub struct RateLimiter {
    redis_client: redis::Client,
    max_requests: u32,
    window_size: Duration,
}

impl RateLimiter {
    pub fn new(redis_client: redis::Client, max_requests: u32, window_size: Duration) -> Self {
        Self {
            redis_client,
            max_requests,
            window_size,
        }
    }
}

impl<S, B> Transform<S, ServiceRequest> for RateLimiter
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = RateLimiterMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(RateLimiterMiddleware {
            service,
            redis_client: self.redis_client.clone(),
            max_requests: self.max_requests,
            window_size: self.window_size
        })
    }
}

pub struct RateLimiterMiddleware<S> {
    service: S,
    redis_client: redis::Client,
    max_requests: u32,
    window_size: Duration,
}

impl<S, B> Service<ServiceRequest> for RateLimiterMiddleware<S>
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
        let redis_client = self.redis_client.clone();
        let max_requests = self.max_requests;
        let window_size = self.window_size;

        let connection_info = req.connection_info().clone();
        let fut = self.service.call(req);

        Box::pin(async move {
            let mut redis_conn = redis_client.get_multiplexed_async_connection().await.map_err(|_| {
                actix_web::error::ErrorInternalServerError("Failed to connect to Redis")
            })?;
            let key = format!("rate_limiter:{}", connection_info.realip_remote_addr().unwrap());
            let count: u32 = redis_conn.get(&key).await.unwrap_or(0);
            let ip: String = connection_info.realip_remote_addr().unwrap().to_string();

            if count >= max_requests {
                return Err(actix_web::error::ErrorTooManyRequests(format!("Rate limit exceeded for IP: {} - {} requests in {} seconds", ip, max_requests, window_size.as_secs())));
            }

            let _: () = redis_conn.incr(&key, 1).await.unwrap();
            let _: () = redis_conn.expire(&key, window_size.as_secs() as usize as i64).await.unwrap();

            let res = fut.await?;
            Ok(res)
        })
    }
}
