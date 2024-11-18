use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};

use super::user::generate_api_key;
use super::user::User;
use crate::utils::jwt::create_jwt;

pub async fn hash_password(password: &str) -> String {
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .expect("Password hashing failed");

    password_hash.to_string()
}

pub async fn verify_password(password: &str, hash: &str) -> bool {
    let argon2 = Argon2::default();
    let parsed_hash = PasswordHash::new(hash).expect("Invalid hash format");
    argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub name: String,
    pub email: String,
    pub password: String,
}

pub async fn register(db_pool: web::Data<sqlx::PgPool>, req: web::Json<RegisterRequest>) -> impl Responder {
    let hashed_password = hash_password(&req.password).await;
    let api_key = generate_api_key().await;

    let regex = regex::Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
    if !regex.is_match(&req.email) {
        return HttpResponse::BadRequest().body("Invalid email address.");
    }

    let user = sqlx::query!(
        "SELECT id FROM users WHERE email = $1",
        req.email
    )
        .fetch_optional(db_pool.get_ref())
        .await;

    if let Ok(Some(_)) = user {
        return HttpResponse::Conflict().body("User with this email already exists.");
    }

    let result = sqlx::query!(
        "INSERT INTO users (name, email, password_hash, api_key, permission)
         VALUES ($1, $2, $3, $4, $5)",
        req.name,
        req.email,
        hashed_password,
        api_key,
        0 // Default permission level
    )
        .execute(db_pool.get_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("User registered successfully."),
        Err(e) => {
            eprintln!("Error: {}", e);
            HttpResponse::InternalServerError().body("Registration failed.")
        }
    }
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

pub async fn login(db_pool: web::Data<sqlx::PgPool>, req: web::Json<LoginRequest>) -> impl Responder {
    let user = sqlx::query_as!(
        User,
        "SELECT id, name, email, api_key, permission, password_hash FROM users WHERE email = $1",
        req.email
    )
        .fetch_optional(db_pool.get_ref())
        .await;

    match user {
        Ok(Some(user)) => {
            if verify_password(&req.password, &user.password_hash).await {
                let token = create_jwt(&user.id.to_string());
                HttpResponse::Ok().json(token)
            } else {
                HttpResponse::Unauthorized().finish()
            }
        }
        Ok(None) => HttpResponse::Unauthorized().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}