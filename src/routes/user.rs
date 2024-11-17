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


#[derive(Serialize, Deserialize)]
struct User {
    id: i32,
    name: String,
    email: String,
    password_hash: String,
    api_key: Option<String>,
    permission: i16,
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
async fn generate_api_key() -> String {
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

////////////////////////////// AUTH //////////////////////////////


pub async fn hash_password(password: &str) -> String {
    let argon2 = Argon2::default(); // Default Argon2 configuration
    let salt = SaltString::generate(&mut OsRng); // Generate a random salt
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt) // Hash the password with the salt
        .expect("Password hashing failed");

    password_hash.to_string() // Convert the hash to a string for storage
}

pub async fn verify_password(password: &str, hash: &str) -> bool {
    let argon2 = Argon2::default();
    let parsed_hash = PasswordHash::new(hash).expect("Invalid hash format"); // Parse the hash
    argon2
        .verify_password(password.as_bytes(), &parsed_hash) // Verify password against the hash
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

    // Insert user into the database
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
                HttpResponse::Ok().json(user)
            } else {
                HttpResponse::Unauthorized().finish()
            }
        }
        Ok(None) => HttpResponse::Unauthorized().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}