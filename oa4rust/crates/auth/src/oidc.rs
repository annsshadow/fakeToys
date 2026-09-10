use axum::{
    extract::{Extension, Query},
    response::{Redirect, Response},
    routing::get,
    Router,
};
use deadpool_postgres::Pool;
use ring::signature::{RsaPublicKeyComponents, RSA_PKCS1_2048_8192_SHA256};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use shared::{
    error::AppError,
    response::row_to_json,
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
        .route("/jaxrs/authentication/oidc/authorize", get(oidc_authorize))
        .route("/jaxrs/authentication/oidc/callback", get(oidc_callback))
}

pub async fn oidc_authorize(Query(req): Query<OidcAuthorizeRequest>) -> Result<Redirect, AppError> {
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
) -> Result<Response, AppError> {
    let token = exchange_code(req.code).await?;
    let claims = verify_id_token(&token.id_token).await?;
    let person = get_or_create_person(&pool, &claims.sub).await?;
    let person_unique = person
        .get("unique_id")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();
    let session = session_manager
        .create_session(person_unique, Uuid::new_v4().to_string())
        .await?;
    Ok(crate::session_response(
        Value::Object(serde_json::Map::from_iter([("person".to_string(), person)])),
        &session.token,
        &session_manager,
    ))
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
    let issuer = std::env::var("OIDC_ISSUER").unwrap_or_default();
    let client_id = std::env::var("OIDC_CLIENT_ID").unwrap_or_default();
    verify_id_token_with_jwks(id_token, &fetch_jwks().await?, &issuer, &client_id)
}

fn verify_id_token_with_jwks(
    id_token: &str,
    jwks: &Value,
    issuer: &str,
    client_id: &str,
) -> Result<OidcClaims, AppError> {
    let mut parts = id_token.split('.');
    let header_b64 = parts.next().ok_or(AppError::Unauthorized)?;
    let payload_b64 = parts.next().ok_or(AppError::Unauthorized)?;
    let signature_b64 = parts.next().ok_or(AppError::Unauthorized)?;
    if parts.next().is_some() {
        return Err(AppError::Unauthorized);
    }

    let header = decode_jwt_part(header_b64)?;
    if header.get("alg").and_then(Value::as_str) != Some("RS256") {
        return Err(AppError::Unauthorized);
    }
    let kid = header
        .get("kid")
        .and_then(Value::as_str)
        .ok_or(AppError::Unauthorized)?;
    let (n, e) = extract_rsa_components(jwks, kid)?;
    let signature = base64url_decode(signature_b64)?;
    RsaPublicKeyComponents { n: &n, e: &e }
        .verify(
            &RSA_PKCS1_2048_8192_SHA256,
            format!("{header_b64}.{payload_b64}").as_bytes(),
            &signature,
        )
        .map_err(|_| AppError::Unauthorized)?;

    let claims: OidcClaims = serde_json::from_slice(&base64url_decode(payload_b64)?)
        .map_err(|_| AppError::Unauthorized)?;
    validate_claims(&claims, issuer, client_id)?;
    Ok(claims)
}

fn validate_claims(
    claims: &OidcClaims,
    issuer: &str,
    client_id: &str,
) -> Result<(), AppError> {
    if claims.iss != issuer || claims.aud != client_id {
        return Err(AppError::Unauthorized);
    }

    let now = chrono::Utc::now().timestamp() as usize;
    if claims.exp <= now {
        return Err(AppError::Unauthorized);
    }
    Ok(())
}

fn decode_jwt_part(part: &str) -> Result<Value, AppError> {
    let bytes = base64url_decode(part)?;
    serde_json::from_slice(&bytes).map_err(|_| AppError::Unauthorized)
}

fn base64url_decode(input: &str) -> Result<Vec<u8>, AppError> {
    use base64::Engine as _;

    base64::engine::general_purpose::URL_SAFE_NO_PAD
        .decode(input)
        .or_else(|_| base64::engine::general_purpose::URL_SAFE.decode(input))
        .map_err(|_| AppError::Unauthorized)
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
) -> Result<(Vec<u8>, Vec<u8>), AppError> {
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
            .and_then(Value::as_str)
            .ok_or(AppError::Internal)
            .and_then(base64url_decode)?;
        let e = key
            .get("e")
            .and_then(Value::as_str)
            .ok_or(AppError::Internal)
            .and_then(base64url_decode)?;
        if n.is_empty() || e.is_empty() {
            return Err(AppError::Unauthorized);
        }

        return Ok((n, e));
    }

    Err(AppError::Unauthorized)
}

pub(crate) async fn get_or_create_person(pool: &Pool, sub: &str) -> Result<Value, AppError> {
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
            &[
                &new_id,
                &unique_id,
                &format!("OIDC User {}", sub),
                &"{bcrypt}$2b$12$dummy",
            ],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(new_id)),
        ("unique_id".to_string(), Value::String(unique_id)),
    ])))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_N: &str = "2LWCJow7CWbkfYUbkVUpAKQGSXLIGVhUW8DZY_G1r2P8rrbb6QlIKjr0TUh7IffnM1NNxUknxekdJ69Nbad6Vh6FoQfVHKoIIvEGKDpDLUtIspOy_CAMGeTqFn5EEfd17llA93YVEd7zHoFfBxlDk0XZ3w71atNR5JzKSaozok2vHo5mECRVtexQUdp4nYuTm9rZL68LaqlUCOqFsZBOTOX_0ZrkJpVTBIXQG5O6zIFyl_tnK_tMIS-rCSNE6Hv5pKv4pN7gDwlW0UaA75RYKadsDoOqn43utSjZoLOtAY-ZYbve0zVFoU7pB-XcmGazlg19NLx1EZmIqqNXDMZJpQ";
    const TEST_E: &str = "AQAB";
    const VALID_TOKEN: &str = "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCIsImtpZCI6InRlc3Qta2lkIn0.eyJzdWIiOiJvaWRjLXVzZXItNDIiLCJpc3MiOiJodHRwczovL2lkcC50ZXN0LmV4YW1wbGUuY29tIiwiYXVkIjoidGVzdC1jbGllbnQtaWQiLCJleHAiOjQxMDI0NDQ4MDB9.TVaB5LADwQ8ot-X3VzjUk46-V57aeYS2mICgJRuyqmOXERzCbJjY7iLj-D1iQVRx50hpavczZxURi49_bdPvWEy0psESz8UfAVTs3DWrj6cXsI4KCgMRVhnv3I5X2mVQ-4denZMRfPm3qqFyv4TC5iCbf0zZXswLflAnmQfYjLXWrr4UJaHDjFd1UaWilORaY0qSY63C-lqWPj-vtsXP5zlY-n_ZBhmcz8CTpyaEzR0BB9E8GgrJMBPH9hJZmdIQSUIS1MvwcdycBJtPv1JcKZUyiuWq1yy6rBpOVlO3qeZsxGOjlkev6bd5ljW5JoRfy3lNzUVA2nW3JSumrlQuHg";
    const EXPIRED_TOKEN: &str = "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCIsImtpZCI6InRlc3Qta2lkIn0.eyJzdWIiOiJvaWRjLXVzZXItNDIiLCJpc3MiOiJodHRwczovL2lkcC50ZXN0LmV4YW1wbGUuY29tIiwiYXVkIjoidGVzdC1jbGllbnQtaWQiLCJleHAiOjk0NjY4NDgwMH0.VjLH7vcov0qVnQfbJ2sJA4nmP9Uas7OexeZKpQA_Q1R7M4pvmS9oSDjU1g50NZR9KUUYdBgK3Q9MNl-WVc4oHGmYRvEA0au7giJWAbHbMgtX8Crzz33kfuEALGWdgPJ30tUVDV9mjAsTTwZuzEZV4Xb2XOtbvDobEKdtDsRa0e8XgFY5D1DIpKsaugNCiMKFIbKyFJz7wjoOGXm_qNO7sGL1GwGiwG_dNe_wdzrX_q_MYUVODtHkyDYajX6IsghTYyMifXEknJnpvOiGVmYz009HDambRJbp87h5D4TArPGkPIT6EZVSCnMs7fte2DKwimiLAxk_RoVygfZ_QVTBWw";

    fn test_jwks() -> Value {
        serde_json::json!({"keys": [{"kty": "RSA", "kid": "test-kid", "n": TEST_N, "e": TEST_E}]})
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

        let (n, e) = crate::oidc::extract_rsa_components(&jwks, "key-1").expect("should find key");
        assert_eq!(e, vec![0x01, 0x00, 0x01]);
        assert!(n.len() > 100, "modulus should decode to RSA key bytes");
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
        let decoded = verify_id_token_with_jwks(
            VALID_TOKEN,
            &test_jwks(),
            "https://idp.test.example.com",
            "test-client-id",
        )
        .expect("fixed RS256 fixture should verify");
        assert_eq!(decoded.sub, "oidc-user-42");
    }

    #[test]
    fn test_oidc_verify_id_token_wrong_issuer_rejected() {
        assert!(
            verify_id_token_with_jwks(
                VALID_TOKEN,
                &test_jwks(),
                "https://evil-idp.example.com",
                "test-client-id",
            )
            .is_err()
        );
    }

    #[test]
    fn test_oidc_verify_id_token_expired_rejected() {
        assert!(
            verify_id_token_with_jwks(
                EXPIRED_TOKEN,
                &test_jwks(),
                "https://idp.test.example.com",
                "test-client-id",
            )
            .is_err()
        );
    }

    #[test]
    fn test_oidc_verify_id_token_tampered_signature_rejected() {
        let mut tampered = VALID_TOKEN.to_string();
        tampered.pop();
        tampered.push('A');

        assert!(
            verify_id_token_with_jwks(
                &tampered,
                &test_jwks(),
                "https://idp.test.example.com",
                "test-client-id",
            )
            .is_err()
        );
    }

    #[tokio::test]
    async fn test_oidc_get_or_create_person_inserts() {
        use shared::testing::is_db_available;

        if !is_db_available().await {
            eprintln!(
                "skipping test_oidc_get_or_create_person_inserts: DATABASE_URL not reachable"
            );
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
            eprintln!(
                "skipping test_oidc_get_or_create_person_existing: DATABASE_URL not reachable"
            );
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
                    &[
                        &"person-oidc-existing",
                        &unique_id,
                        &"Pre-existing OIDC User",
                        &"{bcrypt}$2b$12$dummy",
                    ],
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
}
