use axum::{
    extract::{Extension, Query},
    response::Redirect,
    routing::get,
    Json, Router,
};
use deadpool_postgres::Pool;
use rsa::{pkcs1v15::Pkcs1v15Sign, RsaPublicKey};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use shared::{
    error::AppError,
    response::{row_to_json, ActionResult},
    session::SessionManager,
};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct OidcAuthorizeRequest {
    pub client_id: String,
    pub redirect_uri: String,
    pub response_type: String,
    pub scope: String,
    pub state: String,
}

#[derive(Debug, Deserialize)]
pub struct OidcCallbackRequest {
    pub code: String,
    pub state: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OidcClaims {
    pub sub: String,
    pub iss: String,
    pub aud: String,
    pub exp: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OidcTokenResponse {
    pub access_token: String,
    pub id_token: String,
    pub token_type: String,
}

pub fn oidc_router() -> Router {
    Router::new()
        .route(
            "/jaxrs/authentication/oidc/authorize",
            get(oidc_authorize),
        )
        .route(
            "/jaxrs/authentication/oidc/callback",
            get(oidc_callback),
        )
}

pub async fn oidc_authorize(
    Query(req): Query<OidcAuthorizeRequest>,
) -> Result<Redirect, AppError> {
    let issuer = std::env::var("OIDC_ISSUER").unwrap_or_default();
    let auth_url = format!(
        "https://{}/authorize?client_id={}&redirect_uri={}&response_type=code&scope={}&state={}",
        issuer,
        req.client_id,
        urlencoding::encode(&req.redirect_uri),
        urlencoding::encode(&req.scope),
        req.state,
    );
    Ok(Redirect::to(&auth_url))
}

pub async fn oidc_callback(
    pool: Extension<Pool>,
    session_manager: Extension<SessionManager>,
    Query(req): Query<OidcCallbackRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let token = exchange_code(req.code).await?;
    let claims = verify_id_token(&token.id_token).await?;
    let person = get_or_create_person(&pool, &claims.sub).await?;
    let person_unique = person
        .get("unique_id")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();
    let session_token = session_manager
        .create_session(person_unique, Uuid::new_v4().to_string())
        .await?;
    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("token".to_string(), Value::String(session_token.token)),
        ("person".to_string(), person),
    ])))))
}

async fn exchange_code(code: String) -> Result<OidcTokenResponse, AppError> {
    let client = reqwest::Client::new();
    let issuer = std::env::var("OIDC_ISSUER").unwrap_or_default();
    let redirect_uri = std::env::var("OIDC_REDIRECT_URI").unwrap_or_default();
    let client_id = std::env::var("OIDC_CLIENT_ID").unwrap_or_default();
    let client_secret = std::env::var("OIDC_CLIENT_SECRET").unwrap_or_default();

    let params = [
        ("grant_type", "authorization_code"),
        ("code", code.as_str()),
        ("redirect_uri", redirect_uri.as_str()),
        ("client_id", client_id.as_str()),
        ("client_secret", client_secret.as_str()),
    ];

    let res = client
        .post(format!("{}/token", issuer))
        .form(&params)
        .send()
        .await
        .map_err(|_| AppError::Internal)?;

    let token: OidcTokenResponse = res.json().await.map_err(|_| AppError::Internal)?;
    Ok(token)
}

async fn verify_id_token(id_token: &str) -> Result<OidcClaims, AppError> {
    let jwks: Value = fetch_jwks().await?;

    let header =
        decode_jwt_header(id_token).map_err(|_| AppError::Unauthorized)?;
    let kid = header.get("kid").and_then(|v| v.as_str()).ok_or(AppError::Unauthorized)?;

    let (n_hex, e_hex) = extract_rsa_components(&jwks, kid)?;
    let public_key = build_rsa_public_key(&n_hex, &e_hex).map_err(|_| AppError::Internal)?;

    let signing_input = signing_input(id_token);
    let signature = base64url_decode(
        id_token.split('.').nth(2).ok_or(AppError::Unauthorized)?,
    )
    .map_err(|_| AppError::Unauthorized)?;

    let message_hash = Sha256::digest(signing_input);
    public_key
        .verify(Pkcs1v15Sign::new::<Sha256>(), &message_hash, &signature)
        .map_err(|_| AppError::Unauthorized)?;

    let payload = base64url_decode(id_token.split('.').nth(1).ok_or(AppError::Unauthorized)?)
        .map_err(|_| AppError::Unauthorized)?;
    let claims: OidcClaims =
        serde_json::from_slice(&payload).map_err(|_| AppError::Unauthorized)?;

    let issuer = std::env::var("OIDC_ISSUER").unwrap_or_default();
    let client_id = std::env::var("OIDC_CLIENT_ID").unwrap_or_default();

    if claims.iss != issuer {
        return Err(AppError::Unauthorized);
    }
    if claims.aud != client_id {
        return Err(AppError::Unauthorized);
    }

    let now = chrono::Utc::now().timestamp() as usize;
    if claims.exp <= now {
        return Err(AppError::Unauthorized);
    }

    Ok(claims)
}

fn decode_jwt_header(token: &str) -> Result<Value, AppError> {
    let header_b64 = token.split('.').next().ok_or(AppError::Unauthorized)?;
    let header_bytes = base64url_decode(header_b64).map_err(|_| AppError::Unauthorized)?;
    serde_json::from_slice(&header_bytes).map_err(|_| AppError::Unauthorized)
}

fn signing_input(token: &str) -> Vec<u8> {
    let mut parts = token.splitn(3, '.');
    let header = parts.next().unwrap_or_default();
    let payload = parts.next().unwrap_or_default();
    format!("{}.{}", header, payload).into_bytes()
}

fn base64url_decode(input: &str) -> Result<Vec<u8>, AppError> {
    let normalized = input.replace('-', "+").replace('_', "/");
    let pad = (4 - normalized.len() % 4) % 4;
    let padded = format!("{}{}", normalized, "=".repeat(pad));
    base64::decode_engine(&padded, &base64::engine::general_purpose::STANDARD)
        .map_err(|_| AppError::Internal)
}

fn build_rsa_public_key(n_hex: &str, e_hex: &str) -> Result<RsaPublicKey, AppError> {
    let n_biguint = hex_to_biguint(n_hex).map_err(|_| AppError::Internal)?;
    let e_biguint = hex_to_biguint(e_hex).map_err(|_| AppError::Internal)?;
    RsaPublicKey::new(n_biguint, e_biguint).map_err(|_| AppError::Internal)
}

fn hex_to_biguint(hex_str: &str) -> Result<rsa::BigUint, ()> {
    let normalized = if hex_str.len() % 2 != 0 {
        format!("0{}", hex_str)
    } else {
        hex_str.to_string()
    };
    let bytes = normalized
        .as_bytes()
        .chunks(2)
        .map(|chunk| {
            let hex = std::str::from_utf8(chunk).map_err(|_| ())?;
            u8::from_str_radix(hex, 16).map_err(|_| ())
        })
        .collect::<Result<Vec<u8>, ()>>()?;
    Ok(rsa::BigUint::from_bytes_be(&bytes))
}

/// Convert a JWKS base64url-encoded n value directly to bytes (no hex round-trip).
fn jwks_n_to_bytes(n_b64: &str) -> Result<Vec<u8>, ()> {
    let normalized = n_b64.replace('-', "+").replace('_', "/");
    let pad = (4 - normalized.len() % 4) % 4;
    let padded = format!("{}{}", normalized, "=".repeat(pad));
    base64::Engine::decode(&base64::engine::general_purpose::STANDARD, padded).map_err(|_| ())
}

/// Convert a JWKS base64url-encoded e value directly to bytes.
fn jwks_e_to_bytes(e_b64: &str) -> Result<Vec<u8>, ()> {
    let normalized = e_b64.replace('-', "+").replace('_', "/");
    let pad = (4 - normalized.len() % 4) % 4;
    let padded = format!("{}{}", normalized, "=".repeat(pad));
    base64::Engine::decode(&base64::engine::general_purpose::STANDARD, padded).map_err(|_| ())
}

async fn fetch_jwks() -> Result<Value, AppError> {
    let issuer = std::env::var("OIDC_ISSUER").unwrap_or_default();
    let jwks_url = format!("{}/.well-known/jwks.json", issuer);

    let client = reqwest::Client::new();
    let res = client
        .get(&jwks_url)
        .send()
        .await
        .map_err(|_| AppError::Internal)?;

    res.json().await.map_err(|_| AppError::Internal)
}

pub(crate) fn extract_rsa_components(
    jwks: &Value,
    kid: &str,
) -> Result<(String, String), AppError> {
    let keys = jwks
        .get("keys")
        .and_then(|k| k.as_array())
        .ok_or(AppError::Internal)?;

    for key in keys {
        if key.get("kty").and_then(|v| v.as_str()) != Some("RSA") {
            continue;
        }
        if key.get("kid").and_then(|v| v.as_str()) != Some(kid) {
            continue;
        }

        let n = key
            .get("n")
            .and_then(|v| v.as_str())
            .ok_or(AppError::Internal)?
            .to_string();
        let e = key
            .get("e")
            .and_then(|v| v.as_str())
            .ok_or(AppError::Internal)?
            .to_string();

        return Ok((n, e));
    }

    Err(AppError::Unauthorized)
}

pub(crate) async fn get_or_create_person(
    pool: &Pool,
    sub: &str,
) -> Result<Value, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let unique_id = format!("oidc_{}", sub);

    let row = client
        .query_opt(
            "SELECT id, unique_id, name, mobile, email, icon FROM auth_person WHERE unique_id = $1",
            &[&unique_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if let Some(r) = row {
        return Ok(row_to_json(&r));
    }

    let new_id = Uuid::new_v4().to_string();
    client
        .execute(
            "INSERT INTO auth_person (id, unique_id, name, password_hash) VALUES ($1, $2, $3, $4)",
            &[&new_id, &unique_id, &format!("OIDC User {}", sub), &"{bcrypt}$2b$12$dummy"],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(new_id)),
        ("unique_id".to_string(), Value::String(unique_id)),
    ])))
}

#[cfg(test)]
pub(crate) async fn verify_id_token_with_jwks(
    id_token: &str,
    jwks_override: &Value,
) -> Result<OidcClaims, AppError> {
    let header = decode_jwt_header(id_token).map_err(|_| AppError::Unauthorized)?;
    let kid = header.get("kid").and_then(|v| v.as_str()).ok_or(AppError::Unauthorized)?;
    let (n_hex, e_hex) = extract_rsa_components(jwks_override, kid)?;
    let public_key = build_rsa_public_key(&n_hex, &e_hex).map_err(|_| AppError::Internal)?;

    let signing_input_val = signing_input(id_token);
    let signature = base64url_decode(
        id_token.split('.').nth(2).ok_or(AppError::Unauthorized)?,
    )
    .map_err(|_| AppError::Unauthorized)?;

    let message_hash = Sha256::digest(&signing_input_val);
    public_key
        .verify(Pkcs1v15Sign::new::<Sha256>(), &message_hash, &signature)
        .map_err(|_| AppError::Unauthorized)?;

    let payload = base64url_decode(
        id_token.split('.').nth(1).ok_or(AppError::Unauthorized)?,
    )
    .map_err(|_| AppError::Unauthorized)?;
    let claims: OidcClaims =
        serde_json::from_slice(&payload).map_err(|_| AppError::Unauthorized)?;

    let issuer = std::env::var("OIDC_ISSUER").unwrap_or_default();
    let client_id = std::env::var("OIDC_CLIENT_ID").unwrap_or_default();

    if claims.iss != issuer || claims.aud != client_id {
        return Err(AppError::Unauthorized);
    }

    let now = chrono::Utc::now().timestamp() as usize;
    if claims.exp <= now {
        return Err(AppError::Unauthorized);
    }

    Ok(claims)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rsa::{
        pkcs1v15::Pkcs1v15Sign,
        traits::PublicKeyParts,
        RsaPrivateKey,
    };

    fn generate_test_jwks(
        kid: &str,
        private_key: &RsaPrivateKey,
    ) -> (Value, String, String) {
        let public_key = RsaPrivateKey::to_public_key(private_key);
        let n_hex = public_key.n().to_str_radix(16);
        let e_hex = public_key.e().to_str_radix(16);

        let jwks = serde_json::json!({
            "keys": [{
                "kty": "RSA",
                "kid": kid,
                "n": n_hex,
                "e": e_hex,
            }]
        });
        (jwks, n_hex, e_hex)
    }

    fn encode_test_jwt(
        private_key: &RsaPrivateKey,
        claims: &OidcClaims,
        kid: &str,
    ) -> String {
        let header = serde_json::json!({"alg":"RS256","typ":"JWT","kid":kid});
        let header_b64 =
            base64::encode_engine(serde_json::to_vec(&header).unwrap(), &base64::engine::general_purpose::URL_SAFE);
        let payload_b64 =
            base64::encode_engine(serde_json::to_vec(claims).unwrap(), &base64::engine::general_purpose::URL_SAFE);
        let signing_input = format!("{}.{}", header_b64, payload_b64);
        let hash = Sha256::digest(signing_input.as_bytes());
        let sig_bytes = private_key
            .sign(Pkcs1v15Sign::new::<Sha256>(), &hash)
            .expect("sign test JWT");
        let _public_key = RsaPrivateKey::to_public_key(&private_key);
        let _verify_ok = _public_key.verify(Pkcs1v15Sign::new::<Sha256>(), &hash, &sig_bytes).is_ok();
        let sig_b64 =
            base64::encode_engine(sig_bytes, &base64::engine::general_purpose::URL_SAFE);
        format!("{}.{}.{}", header_b64, payload_b64, sig_b64)
    }

    #[test]
    fn test_extract_rsa_components_valid() {
        let jwks = serde_json::json!({
            "keys": [
                {
                    "kty": "RSA",
                    "kid": "key-1",
                    "n": "0vx7agoebGcQSuuPiLJXZptN9nndrQmbXEps2aiAFbWhM78LhWx4cbbfAAtVT86zwu1RK7aPFFxuhDR1L6tSoc_BJECPebWKRXjBZCiFV4Xq0w5Nkq9MnHDe4o0aS7B7j7MBD-r6kuLdHrhHKn3k9PYp0zLlMDWTnAWk0OLlKdtc",
                    "e": "AQAB"
                }
            ]
        });

        let (n, e) =
            crate::oidc::extract_rsa_components(&jwks, "key-1").expect("should find key");
        assert_eq!(e, "AQAB");
        assert!(n.len() > 100, "modulus should be a long base64url string");
    }

    #[test]
    fn test_extract_rsa_components_kid_mismatch() {
        let jwks = serde_json::json!({
            "keys": [
                {
                    "kty": "RSA",
                    "kid": "other-key",
                    "n": "testn",
                    "e": "AQAB"
                }
            ]
        });

        assert!(crate::oidc::extract_rsa_components(&jwks, "key-1").is_err());
    }

    #[test]
    fn test_oidc_verify_id_token_valid() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let _guard = EnvGuard::new()
                .set("OIDC_ISSUER", "https://idp.test.example.com")
                .set("OIDC_CLIENT_ID", "test-client-id");

            let private_key =
                RsaPrivateKey::new(&mut rand::thread_rng(), 2048).expect("generate RSA key");

            // Raw sign/verify test
            let msg = b"test message for rsa";
            let msg_hash = Sha256::digest(msg);
            let sig1 = private_key
                .sign(Pkcs1v15Sign::new::<Sha256>(), &msg_hash)
                .expect("sign");
            let pub_key1 = RsaPrivateKey::to_public_key(&private_key);
            let ok1 = pub_key1.verify(Pkcs1v15Sign::new::<Sha256>(), &msg_hash, &sig1);
            eprintln!("DEBUG raw ok={:?} sig[0..4]={:02x?}", ok1.is_ok(), &sig1[..4]);

            // Second sign with same inputs
            let sig2 = private_key
                .sign(Pkcs1v15Sign::new::<Sha256>(), &msg_hash)
                .expect("sign2");
            let ok2 = pub_key1.verify(Pkcs1v15Sign::new::<Sha256>(), &msg_hash, &sig2);
            eprintln!("DEBUG raw2 ok={:?} sig[0..4]={:02x?} same={}", ok2.is_ok(), &sig2[..4], sig1 == sig2);

            let (jwks, _, _) = generate_test_jwks("test-kid", &private_key);

            let claims = OidcClaims {
                sub: "oidc-user-42".to_string(),
                iss: "https://idp.test.example.com".to_string(),
                aud: "test-client-id".to_string(),
                exp: (chrono::Utc::now() + chrono::Duration::hours(1)).timestamp() as usize,
            };

            let token = encode_test_jwt(&private_key, &claims, "test-kid");

            let decoded =
                crate::oidc::verify_id_token_with_jwks(&token, &jwks)
                    .await
                    .expect("id_token should verify");
            assert_eq!(decoded.sub, "oidc-user-42");
            assert_eq!(decoded.iss, "https://idp.test.example.com");
            assert_eq!(decoded.aud, "test-client-id");
        });
    }

    #[test]
    fn test_oidc_verify_id_token_wrong_issuer_rejected() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let _guard = EnvGuard::new()
                .set("OIDC_ISSUER", "https://idp.test.example.com")
                .set("OIDC_CLIENT_ID", "test-client-id");

            let private_key =
                RsaPrivateKey::new(&mut rand::thread_rng(), 2048).expect("generate RSA key");
            let (jwks, _, _) = generate_test_jwks("test-kid", &private_key);

            let claims = OidcClaims {
                sub: "oidc-user-42".to_string(),
                iss: "https://evil-idp.example.com".to_string(),
                aud: "test-client-id".to_string(),
                exp: (chrono::Utc::now() + chrono::Duration::hours(1)).timestamp() as usize,
            };

            let token = encode_test_jwt(&private_key, &claims, "test-kid");

            assert!(
                crate::oidc::verify_id_token_with_jwks(&token, &jwks)
                    .await
                    .is_err(),
                "token with wrong issuer should be rejected"
            );
        });
    }

    #[test]
    fn test_oidc_verify_id_token_expired_rejected() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let _guard = EnvGuard::new()
                .set("OIDC_ISSUER", "https://idp.test.example.com")
                .set("OIDC_CLIENT_ID", "test-client-id");

            let private_key =
                RsaPrivateKey::new(&mut rand::thread_rng(), 2048).expect("generate RSA key");
            let (jwks, _, _) = generate_test_jwks("test-kid", &private_key);

            let claims = OidcClaims {
                sub: "oidc-user-42".to_string(),
                iss: "https://idp.test.example.com".to_string(),
                aud: "test-client-id".to_string(),
                exp: (chrono::Utc::now() - chrono::Duration::hours(1)).timestamp() as usize,
            };

            let token = encode_test_jwt(&private_key, &claims, "test-kid");

            assert!(
                crate::oidc::verify_id_token_with_jwks(&token, &jwks)
                    .await
                    .is_err(),
                "expired token should be rejected"
            );
        });
    }

    async fn verify_id_token_with_jwks(
        id_token: &str,
        jwks_override: &Value,
    ) -> Result<OidcClaims, AppError> {
        let header =
            decode_jwt_header(id_token).map_err(|_| AppError::Unauthorized)?;
        let kid = header.get("kid").and_then(|v| v.as_str()).ok_or(AppError::Unauthorized)?;
        let (n_hex, e_hex) = extract_rsa_components(jwks_override, kid)?;
        let public_key = build_rsa_public_key(&n_hex, &e_hex).map_err(|_| AppError::Internal)?;

        let signing_input = signing_input(id_token);
        let signature = base64url_decode(
            id_token.split('.').nth(2).ok_or(AppError::Unauthorized)?,
        )
        .map_err(|_| AppError::Unauthorized)?;

        let message_hash = Sha256::digest(signing_input);
        public_key
            .verify(Pkcs1v15Sign::new::<Sha256>(), &message_hash, &signature)
            .map_err(|_| AppError::Unauthorized)?;

        let payload = base64url_decode(
            id_token.split('.').nth(1).ok_or(AppError::Unauthorized)?,
        )
        .map_err(|_| AppError::Unauthorized)?;
        let claims: OidcClaims =
            serde_json::from_slice(&payload).map_err(|_| AppError::Unauthorized)?;

        let issuer = std::env::var("OIDC_ISSUER").unwrap_or_default();
        let client_id = std::env::var("OIDC_CLIENT_ID").unwrap_or_default();

        if claims.iss != issuer || claims.aud != client_id {
            return Err(AppError::Unauthorized);
        }

        let now = chrono::Utc::now().timestamp() as usize;
        if claims.exp <= now {
            return Err(AppError::Unauthorized);
        }

        Ok(claims)
    }

    #[tokio::test]
    async fn test_oidc_get_or_create_person_inserts() {
        use shared::testing::is_db_available;

        if !is_db_available().await {
            eprintln!("skipping test_oidc_get_or_create_person_inserts: DATABASE_URL not reachable");
            return;
        }

        let pool = shared::testing::test_pool();
        let client = pool.get().await.ok();

        if let Some(c) = &client {
            let _ = c
                .execute(
                    "DELETE FROM auth_person WHERE unique_id = $1",
                    &[&"oidc_test_create_user_001"],
                )
                .await;
        }

        let result = crate::oidc::get_or_create_person(&pool, "test_create_user_001")
            .await
            .expect("get_or_create_person should succeed");

        assert_eq!(
            result.get("unique_id").and_then(|v| v.as_str()),
            Some("oidc_test_create_user_001")
        );
        assert!(
            result.get("id").and_then(|v| v.as_str()).is_some(),
            "new person should have an id"
        );
    }

    #[tokio::test]
    async fn test_oidc_get_or_create_person_existing() {
        use shared::testing::is_db_available;

        if !is_db_available().await {
            eprintln!("skipping test_oidc_get_or_create_person_existing: DATABASE_URL not reachable");
            return;
        }

        let pool = shared::testing::test_pool();
        let client = pool.get().await.ok();

        let unique_id = "oidc_test_existing_user_001";

        if let Some(c) = &client {
            let _ = c
                .execute(
                    "INSERT INTO auth_person (id, unique_id, name, password_hash) \
                     VALUES ($1, $2, $3, $4) \
                     ON CONFLICT (unique_id) DO UPDATE SET name = EXCLUDED.name",
                    &[&"person-oidc-existing", &unique_id, &"Pre-existing OIDC User", &"{bcrypt}$2b$12$dummy"],
                )
                .await;
        }

        let result = crate::oidc::get_or_create_person(&pool, "test_existing_user_001")
            .await
            .expect("get_or_create_person should succeed");

        assert_eq!(
            result.get("unique_id").and_then(|v| v.as_str()),
            Some(unique_id)
        );
        assert_eq!(
            result.get("name").and_then(|v| v.as_str()),
            Some("Pre-existing OIDC User")
        );
    }

    struct EnvGuard {
        vars: Vec<(&'static str, Option<String>)>,
    }

    impl EnvGuard {
        fn new() -> Self {
            Self { vars: Vec::new() }
        }
        fn set(mut self, key: &'static str, value: impl Into<String>) -> Self {
            let prev = std::env::var(key).ok();
            self.vars.push((key, prev));
            std::env::set_var(key, value.into());
            self
        }
    }

    impl Drop for EnvGuard {
        fn drop(&mut self) {
            for (key, prev) in self.vars.drain(..).rev() {
                match prev {
                    Some(v) => std::env::set_var(key, v),
                    None => std::env::remove_var(key),
                }
            }
        }
    }
}
