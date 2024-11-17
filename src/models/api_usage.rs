use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ApiUsage {
    pub id: i32,
    pub user_id: i32,
    pub api_key: String,
    pub request_path: String,
    pub request_method: String,
    pub request_ip: String,
}