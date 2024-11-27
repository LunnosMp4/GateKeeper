use redis::Client;

pub fn create_redis_client(redis_url: &str) -> Client {
    Client::open(redis_url).expect("Failed to create Redis client")
}
