use actix_web::http::StatusCode;
use serde::{Deserialize, Serialize};
use time::PrimitiveDateTime;

#[derive(Serialize, Deserialize)]
pub struct ApiUsage {
    pub id: i32,
    pub user_id: i32,
    pub api_key: String,
    pub request_path: String,
    pub request_method: String,
    pub request_time: PrimitiveDateTime,
    pub request_ip: String,
    pub status_code: i32,
}