use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;


#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub api_key: Option<String>,
    pub permission: i16,
}

/**
 * Get all users from the database
 *
 * @param db_pool: web::Data<PgPool>
 *
 * @return impl Responder
 */
pub async fn get_users(db_pool: web::Data<PgPool>) -> impl Responder {
    let result = sqlx::query_as!(User, "SELECT id, name, email, api_key, permission, password_hash FROM users")
        .fetch_all(&**db_pool)
        .await;

    match result {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

/**
 * Get user by ID
 *
 * @param db_pool: web::Data<PgPool>
 *
 * @param path: web::Path<i32>
 *
 * @return impl Responder
 */
pub async fn get_user_by_id(db_pool: web::Data<PgPool>, path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner();

    let result = sqlx::query_as!(User, "SELECT id, name, email, api_key, permission, password_hash FROM users WHERE id = $1",
        id
    )
        .fetch_one(&**db_pool)
        .await;

    match result {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

// Define the NewUser struct
#[derive(Deserialize)]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub api_key: Option<String>,
    pub permission: i16,
}

// Generate a new API key
pub async fn generate_api_key() -> String {
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};

    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect()
}

/**
 * Add a new user
 *
 * @param db_pool: web::Data<PgPool>
 *
 * @param new_user: web::Json<NewUser>
 *
 * @return impl Responder
 */
pub async fn add_user(db_pool: web::Data<PgPool>, new_user: web::Json<NewUser>) -> impl Responder {
    let api_key = generate_api_key().await;

    let result = sqlx::query!("INSERT INTO users (name, email, api_key, permission) VALUES ($1, $2, $3, $4) RETURNING id",
        new_user.name,
        new_user.email,
        api_key,
        0
    )
        .fetch_one(&**db_pool)
        .await;

    match result {
        Ok(record) => HttpResponse::Created().json(record.id),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

/**
 * Delete user by ID
 *
 * @param db_pool: web::Data<PgPool>
 *
 * @param path: web::Path<i32>
 *
 * @return impl Responder
 */
pub async fn delete_user(db_pool: web::Data<PgPool>, path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner();

    let result = sqlx::query!("DELETE FROM users WHERE id = $1",
        id
    )
        .execute(&**db_pool)
        .await;

    match result {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

/**
 * Refresh API key
 *
 * @param db_pool: web::Data<PgPool>
 *
 * @param path: web::Path<i32>
 *
 * @return impl Responder
 */
pub async fn refresh_api_key(db_pool: web::Data<PgPool>, path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner();
    let api_key = generate_api_key().await;

    let result = sqlx::query!("UPDATE users SET api_key = $1 WHERE id = $2",
        api_key,
        id
    )
        .execute(&**db_pool)
        .await;

    match result {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn change_permission(db_pool: web::Data<PgPool>, path: web::Path<(i32, i32)>) -> impl Responder {
    let (id, permission) = path.into_inner();

    if permission < 0 || permission > 1 {
        return HttpResponse::BadRequest().finish();
    }

    let result = sqlx::query!("UPDATE users SET permission = $1 WHERE id = $2",
        permission as i16,
        id
    )
        .execute(&**db_pool)
        .await;

    match result {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}