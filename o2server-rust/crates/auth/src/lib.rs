use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    middleware::from_fn,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use chrono::{DateTime, Duration, Utc};
use deadpool_postgres::Pool;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

pub mod model;
pub mod password;
pub mod person;
pub mod secret;

// --- Request/Response DTOs ---

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub credential: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub person: PersonInfo,
}

#[derive(Debug, Serialize, Clone)]
pub struct PersonInfo {
    pub unique: String,
    pub name: String,
    pub mobile: Option<String>,
}

// --- Session Management (Rust-side independent) ---

#[derive(Clone, Serialize, Deserialize)]
pub struct Session {
    pub token: String,
    pub person_unique: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

#[derive(Clone)]
pub struct SessionManager {
    pub sessions: Arc<RwLock<std::collections::HashMap<String, Session>>>,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(RwLock::new(std::collections::HashMap::new())),
        }
    }

    pub async fn create_session(&self, person_unique: String, token: String) -> Session {
        let now = Utc::now();
        let session = Session {
            token: token.clone(),
            person_unique,
            created_at: now,
            expires_at: now + Duration::hours(2),
        };

        self.sessions.write().await.insert(token.clone(), session.clone());
        session
    }

    pub async fn validate_session(&self, token: &str) -> Option<Session> {
        let sessions = self.sessions.read().await;
        sessions.get(token).cloned()
    }
}

// --- Rate Limiting ---

#[derive(Clone)]
pub struct RateLimiter {
    pub attempts: Arc<RwLock<std::collections::HashMap<String, (i32, DateTime<Utc>)>>>,
}

impl RateLimiter {
    pub fn new() -> Self {
        Self {
            attempts: Arc::new(RwLock::new(std::collections::HashMap::new())),
        }
    }

    pub async fn check_rate_limit(&self, key: &str, max_attempts: i32, window_minutes: i64) -> Result<(), AppError> {
        let mut attempts = self.attempts.write().await;
        let now = Utc::now();

        if let Some((count, last_attempt)) = attempts.get(key) {
            let elapsed = now - *last_attempt;
            if elapsed.num_minutes() < window_minutes {
                if *count >= max_attempts {
                    return Err(AppError::BadRequest(
                        format!("rate limit exceeded: {} attempts in last {} minutes", count, window_minutes)
                    ));
                }
            }
        }

        let count = attempts.get(key).map(|(c, _)| c + 1).unwrap_or(1);
        attempts.insert(key.to_string(), (count, now));
        Ok(())
    }

    pub async fn record_failure(&self, key: &str) {
        let mut attempts = self.attempts.write().await;
        let now = Utc::now();
        let count = attempts.get(key).map(|(c, _)| c + 1).unwrap_or(1);
        attempts.insert(key.to_string(), (count, now));
    }

    pub async fn reset(&self, key: &str) {
        self.attempts.write().await.remove(key);
    }
}

// --- Authentication Handlers ---

#[axum::debug_handler]
pub async fn login(
    pool: Extension<Pool>,
    rate_limiter: Extension<RateLimiter>,
    session_manager: Extension<SessionManager>,
    axum::extract::Json(req): axum::extract::Json<LoginRequest>,
) -> Result<Json<ActionResult<LoginResponse>>, AppError> {
    let ip = "127.0.0.1";
    rate_limiter.check_rate_limit(ip, 5, 1).await?;

    if req.credential.is_empty() || req.password.is_empty() {
        rate_limiter.record_failure(ip).await;
        return Ok(Json(ActionResult::error("invalid credentials")));
    }

    let client = pool.get().await.map_err(|e| {
        AppError::Internal
    })?;

    let row = client
        .query_one(
            "SELECT id, unique_id, name, mobile, password_hash, locked FROM auth_person WHERE unique_id = $1 AND locked = false",
            &[&req.credential],
        )
        .await
        .map_err(|_| AppError::Unauthorized)?;

    let person_id: String = row.get("id");
    let person_unique: String = row.get("unique_id");
    let person_name: String = row.get("name");
    let person_mobile: Option<String> = row.get("mobile");
    let password_hash: String = row.get("password_hash");

    // 验证密码（支持 MD5 和 DES 加密）
    let valid = password::verify_password(&req.password, &password_hash, "", None);
    if !valid {
        rate_limiter.record_failure(ip).await;
        return Ok(Json(ActionResult::error("invalid credentials")));
    }

    let token = Uuid::new_v4().to_string();
    let session = session_manager.create_session(person_unique.clone(), token.clone()).await;

    rate_limiter.reset(ip).await;

    let response = ActionResult::success(LoginResponse {
        token: session.token,
        person: PersonInfo {
            unique: person_unique,
            name: person_name,
            mobile: person_mobile,
        },
    });

    Ok(Json(response))
}

pub async fn logout(
    _session_manager: Extension<SessionManager>,
    _payload: Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([("message".to_string(), Value::String("logged out".to_string()))])))))
}

pub async fn whoami(
    _session_manager: Extension<SessionManager>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([("authenticated".to_string(), Value::Bool(false))])))))
}

pub async fn captcha() -> Result<Json<ActionResult<Value>>, AppError> {
    let captcha_id = Uuid::new_v4().to_string();
    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("captchaId".to_string(), Value::String(captcha_id)),
        ("image".to_string(), Value::String("base64-encoded-image-placeholder".to_string())),
    ])))))
}

pub async fn bind(
    _payload: Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("bound".to_string(), Value::Bool(true)),
    ])))))
}

pub async fn oauth(
    _payload: Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("oauthUrl".to_string(), Value::String("https://oauth.example.com/authorize".to_string())),
    ])))))
}

pub async fn refresh(
    _payload: Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("token".to_string(), Value::String(Uuid::new_v4().to_string())),
    ])))))
}

pub async fn code(
    _payload: Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("code".to_string(), Value::String(Uuid::new_v4().to_string())),
    ])))))
}

pub async fn unit_list() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(1))),
        ("data".to_string(), Value::Array(vec![
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String("unit-root".to_string())),
                ("name".to_string(), Value::String("root".to_string())),
            ]))
        ])),
    ])))))
}

pub async fn role_list() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(2))),
        ("data".to_string(), Value::Array(vec![
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String("role-admin".to_string())),
                ("name".to_string(), Value::String("admin".to_string())),
            ])),
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String("role-user".to_string())),
                ("name".to_string(), Value::String("user".to_string())),
            ])),
        ])),
    ])))))
}

pub async fn group_list() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(0))),
        ("data".to_string(), Value::Array(vec![])),
    ])))))
}

// --- Router ---

pub fn router(pool: Pool) -> Router {
    let rate_limiter = RateLimiter::new();
    let session_manager = SessionManager::new();

    Router::new()
        .route("/jaxrs/authentication/login", post(login))
        .route("/jaxrs/authentication/logout", post(logout))
        .route("/jaxrs/authentication/who", get(whoami))
        .route("/jaxrs/authentication/captcha", get(captcha))
        .route("/jaxrs/authentication/bind", post(bind))
        .route("/jaxrs/authentication/oauth", post(oauth))
        .route("/jaxrs/authentication/refresh", post(refresh))
        .route("/jaxrs/authentication/code", post(code))
        .route("/jaxrs/person/{flag}", get(person::get))
        .route("/jaxrs/person/list", get(person::list))
        .route("/jaxrs/unit/list", get(unit_list))
        .route("/jaxrs/role/list", get(role_list))
        .route("/jaxrs/group/list", get(group_list))
        .merge(secret::router())
        .layer(Extension(pool))
        .layer(Extension(rate_limiter))
        .layer(Extension(session_manager))
}
