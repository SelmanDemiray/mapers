use axum::{
    extract::Extension,
    http::{HeaderMap, StatusCode},
    response::Json,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::Arc;
use bcrypt::{hash, verify, DEFAULT_COST};

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    pub success: bool,
    pub message: String,
    pub username: Option<String>,
    pub user_id: Option<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct SessionRequest {
    pub username: String,
}

#[derive(Serialize, Deserialize)]
pub struct ConnectedUser {
    pub username: String,
    pub ip_address: String,
    pub last_seen: String,
}

#[derive(Serialize)]
pub struct ConnectedUsersResponse {
    pub users: Vec<ConnectedUser>,
}

#[derive(Serialize, Deserialize)]
pub struct DeleteAccountRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct PreviousUsernamesResponse {
    pub usernames: Vec<String>,
}

/// Extract IP address from headers, handling Cloudflare headers
fn extract_ip_address(headers: &HeaderMap) -> String {
    // Check Cloudflare headers first (when behind Cloudflare Tunnel)
    if let Some(cf_connecting_ip) = headers.get("cf-connecting-ip") {
        if let Ok(ip) = cf_connecting_ip.to_str() {
            return ip.to_string();
        }
    }
    
    // Check X-Forwarded-For header
    if let Some(xff) = headers.get("x-forwarded-for") {
        if let Ok(ip) = xff.to_str() {
            // X-Forwarded-For can contain multiple IPs, take the first one
            return ip.split(',').next().unwrap_or("").trim().to_string();
        }
    }
    
    // Check X-Real-IP header
    if let Some(xri) = headers.get("x-real-ip") {
        if let Ok(ip) = xri.to_str() {
            return ip.to_string();
        }
    }
    
    // Fallback
    "unknown".to_string()
}

/// Authenticate user with username and password from database
pub async fn login(
    Extension(pool): Extension<Arc<PgPool>>,
    Json(login_req): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    // Validate username (must be non-empty and reasonable length)
    if login_req.username.is_empty() || login_req.username.len() > 50 {
        return Ok(Json(LoginResponse {
            success: false,
            message: "Invalid username".to_string(),
            username: None,
            user_id: None,
        }));
    }
    
    // Check if user exists in database
    let user_result = sqlx::query_as::<_, (i32, String)>(
        "SELECT id, password_hash FROM users WHERE username = $1"
    )
    .bind(&login_req.username)
    .fetch_optional(pool.as_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error during login: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    
    match user_result {
        Some((user_id, password_hash)) => {
            // User exists, verify password
            match verify(&login_req.password, &password_hash) {
                Ok(true) => {
    Ok(Json(LoginResponse {
        success: true,
        message: "Login successful".to_string(),
        username: Some(login_req.username),
                        user_id: Some(user_id),
    }))
                }
                Ok(false) => {
                    Ok(Json(LoginResponse {
                        success: false,
                        message: "Invalid password".to_string(),
                        username: None,
                        user_id: None,
                    }))
                }
                Err(e) => {
                    eprintln!("Password verification error: {}", e);
                    Ok(Json(LoginResponse {
                        success: false,
                        message: "Authentication error".to_string(),
                        username: None,
                        user_id: None,
                    }))
                }
            }
        }
        None => {
            // User doesn't exist, create new user with hashed password
            let password_hash = hash(&login_req.password, DEFAULT_COST)
                .map_err(|e| {
                    eprintln!("Password hashing error: {}", e);
                    StatusCode::INTERNAL_SERVER_ERROR
                })?;
            
            // Generate a dummy email (username@gamersunite.local) since email is required but not used
            let email = format!("{}@gamersunite.local", login_req.username);
            
            let result = sqlx::query(
                "INSERT INTO users (username, email, password_hash) 
                 VALUES ($1, $2, $3)
                 ON CONFLICT (username) DO NOTHING"
            )
            .bind(&login_req.username)
            .bind(&email)
            .bind(&password_hash)
            .execute(pool.as_ref())
            .await;
            
            match result {
                Ok(_result) => {
                    // Get the created user's ID
                    let created_user_id = sqlx::query_as::<_, (i32,)>(
                        "SELECT id FROM users WHERE username = $1"
                    )
                    .bind(&login_req.username)
                    .fetch_optional(pool.as_ref())
                    .await
                    .ok()
                    .flatten()
                    .map(|(id,)| id);
                    
                    Ok(Json(LoginResponse {
                        success: true,
                        message: "Account created and login successful".to_string(),
                        username: Some(login_req.username),
                        user_id: created_user_id,
                    }))
                }
                Err(sqlx::Error::Database(db_err)) if db_err.constraint().is_some() => {
                    // Username conflict, try to login again (race condition)
                    // This shouldn't happen often, but handle it gracefully
                    let user_result = sqlx::query_as::<_, (i32, String)>(
                        "SELECT id, password_hash FROM users WHERE username = $1"
                    )
                    .bind(&login_req.username)
                    .fetch_optional(pool.as_ref())
                    .await
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
                    
                    if let Some((_, password_hash)) = user_result {
                        let user_id_result = sqlx::query_as::<_, (i32,)>(
                            "SELECT id FROM users WHERE username = $1"
                        )
                        .bind(&login_req.username)
                        .fetch_optional(pool.as_ref())
                        .await
                        .ok()
                        .flatten();
                        
                        match verify(&login_req.password, &password_hash) {
                            Ok(true) => {
                                Ok(Json(LoginResponse {
                                    success: true,
                                    message: "Login successful".to_string(),
                                    username: Some(login_req.username),
                                    user_id: user_id_result.map(|(id,)| id),
                                }))
                            }
                            _ => {
                                Ok(Json(LoginResponse {
                                    success: false,
                                    message: "Invalid password".to_string(),
                                    username: None,
                                    user_id: None,
                                }))
                            }
                        }
                    } else {
                        Ok(Json(LoginResponse {
                            success: false,
                            message: "Account creation failed".to_string(),
                            username: None,
                            user_id: None,
                        }))
                    }
                }
                Err(e) => {
                    eprintln!("Error creating user: {}", e);
                    Err(StatusCode::INTERNAL_SERVER_ERROR)
                }
            }
        }
    }
}

/// Register or update user session with IP address
pub async fn register_session(
    Extension(pool): Extension<Arc<PgPool>>,
    headers: HeaderMap,
    Json(session_req): Json<SessionRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // Extract IP address from headers
    let ip_address = extract_ip_address(&headers);
    
    // Validate username
    if session_req.username.is_empty() || session_req.username.len() > 50 {
        return Err(StatusCode::BAD_REQUEST);
    }
    
    // Upsert session (insert or update last_seen)
    let result = sqlx::query(
        "INSERT INTO active_sessions (username, ip_address, last_seen)
         VALUES ($1, $2, CURRENT_TIMESTAMP)
         ON CONFLICT (username, ip_address)
         DO UPDATE SET last_seen = CURRENT_TIMESTAMP"
    )
    .bind(&session_req.username)
    .bind(&ip_address)
    .execute(pool.as_ref())
    .await;
    
    match result {
        Ok(_) => Ok(Json(serde_json::json!({
            "success": true,
            "message": "Session registered"
        }))),
        Err(e) => {
            eprintln!("Error registering session: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Get list of all connected users
pub async fn get_connected_users(
    Extension(pool): Extension<Arc<PgPool>>,
) -> Result<Json<ConnectedUsersResponse>, StatusCode> {
    // Get users active in the last 5 minutes
    let users = sqlx::query_as::<_, (String, String, chrono::DateTime<chrono::Utc>)>(
        "SELECT username, ip_address, last_seen
         FROM active_sessions
         WHERE last_seen > NOW() - INTERVAL '5 minutes'
         ORDER BY last_seen DESC"
    )
    .fetch_all(pool.as_ref())
    .await
    .map_err(|e| {
        eprintln!("Error fetching connected users: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    
    let connected_users: Vec<ConnectedUser> = users
        .into_iter()
        .map(|(username, ip_address, last_seen)| ConnectedUser {
            username,
            ip_address,
            last_seen: last_seen.to_rfc3339(),
        })
        .collect();
    
    Ok(Json(ConnectedUsersResponse {
        users: connected_users,
    }))
}

/// Clean up old sessions (called periodically)
pub async fn cleanup_old_sessions(pool: Arc<PgPool>) {
    let _ = sqlx::query(
        "DELETE FROM active_sessions
         WHERE last_seen < NOW() - INTERVAL '10 minutes'"
    )
    .execute(pool.as_ref())
    .await;
}

/// Get list of all previous usernames (for quick login)
pub async fn get_previous_usernames(
    Extension(pool): Extension<Arc<PgPool>>,
) -> Result<Json<PreviousUsernamesResponse>, StatusCode> {
    let usernames = sqlx::query_as::<_, (String,)>(
        "SELECT DISTINCT username FROM users ORDER BY created_at DESC LIMIT 50"
    )
    .fetch_all(pool.as_ref())
    .await
    .map_err(|e| {
        eprintln!("Error fetching previous usernames: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    
    let username_list: Vec<String> = usernames
        .into_iter()
        .map(|(username,)| username)
        .collect();
    
    Ok(Json(PreviousUsernamesResponse {
        usernames: username_list,
    }))
}

/// Delete user account and all associated data
pub async fn delete_account(
    Extension(pool): Extension<Arc<PgPool>>,
    Json(delete_req): Json<DeleteAccountRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // Validate username
    if delete_req.username.is_empty() || delete_req.username.len() > 50 {
        return Err(StatusCode::BAD_REQUEST);
    }
    
    // Verify user exists and password is correct
    let user_result = sqlx::query_as::<_, (i32, String)>(
        "SELECT id, password_hash FROM users WHERE username = $1"
    )
    .bind(&delete_req.username)
    .fetch_optional(pool.as_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error during account deletion: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    
    match user_result {
        Some((user_id, password_hash)) => {
            // Verify password
            match verify(&delete_req.password, &password_hash) {
                Ok(true) => {
                    // Password is correct, delete user (CASCADE will handle related data)
                    let result = sqlx::query("DELETE FROM users WHERE id = $1")
                        .bind(user_id)
                        .execute(pool.as_ref())
                        .await;
                    
                    match result {
                        Ok(_) => {
                            // Also clean up active sessions for this user
                            let _ = sqlx::query("DELETE FROM active_sessions WHERE username = $1")
                                .bind(&delete_req.username)
                                .execute(pool.as_ref())
                                .await;
                            
                            Ok(Json(serde_json::json!({
                                "success": true,
                                "message": "Account deleted successfully"
                            })))
                        }
                        Err(e) => {
                            eprintln!("Error deleting account: {}", e);
                            Err(StatusCode::INTERNAL_SERVER_ERROR)
                        }
                    }
                }
                Ok(false) => {
                    Ok(Json(serde_json::json!({
                        "success": false,
                        "message": "Invalid password"
                    })))
                }
                Err(e) => {
                    eprintln!("Password verification error: {}", e);
                    Err(StatusCode::INTERNAL_SERVER_ERROR)
                }
            }
        }
        None => {
            Ok(Json(serde_json::json!({
                "success": false,
                "message": "User not found"
            })))
        }
    }
}

