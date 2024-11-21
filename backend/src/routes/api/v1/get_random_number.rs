use actix_web::{HttpResponse, Responder};

pub async fn get_random_number() -> impl Responder {
    let random_number = rand::random::<i32>();
    HttpResponse::Ok().body(random_number.to_string())
}