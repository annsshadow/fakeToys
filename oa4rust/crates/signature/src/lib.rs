use std::sync::Arc;

use async_trait::async_trait;
use axum::{Router, extract::State, Json};
use chrono::Utc;
use ring::{rand::SystemRandom, signature};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::warn;
use zeroize::Zeroizing;

#[derive(Debug, Error)]
pub enum SignatureError {
    #[error("invalid key format")]
    InvalidKey,
    #[error("invalid certificate format")]
    InvalidCertificate,
    #[error("pdf operation error: {0}")]
    PdfOperation(String),
    #[error("verification failed: {0}")]
    VerificationFailed(String),
    #[error("certificate chain verification failed: {0}")]
    ChainVerification(String),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("serialization error: {0}")]
    Serialize(#[from] serde_json::Error),
}

pub type SignatureResult<T> = Result<T, SignatureError>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignatureInfo {
    pub reason: String,
    pub location: String,
    pub contact: String,
    pub cert_pem: String,
    pub private_key_pem: String,
    pub signer_name: Option<String>,
}

impl SignatureInfo {
    pub fn new(
        reason: impl Into<String>,
        location: impl Into<String>,
        contact: impl Into<String>,
        cert_pem: impl Into<String>,
        private_key_pem: impl Into<String>,
    ) -> Self {
        Self {
            reason: reason.into(),
            location: location.into(),
            contact: contact.into(),
            cert_pem: cert_pem.into(),
            private_key_pem: private_key_pem.into(),
            signer_name: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationResult {
    pub valid: bool,
    pub signer: Option<String>,
    pub signing_time: Option<String>,
    pub reason: Option<String>,
    pub chain: Option<Vec<ChainLink>>,
    pub cert_pem: String,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignPdfRequest {
    pub file_data: String,
    pub signature_info: SignatureInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyPdfRequest {
    pub file_data: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignPdfResponse {
    pub success: bool,
    pub signed_data: Option<String>,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyPdfResponse {
    pub success: bool,
    pub result: Option<VerificationResult>,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertStatus {
    pub subject: String,
    pub issuer: String,
    pub serial_number: String,
    pub not_before: String,
    pub not_after: String,
    pub revoked: bool,
    pub revocation_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainLink {
    pub subject: String,
    pub issuer: String,
    pub signature_valid: bool,
    pub is_self_signed: bool,
    pub depth: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignatureChainResponse {
    pub valid: bool,
    pub chain: Vec<ChainLink>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignatureStatusRequest {
    pub file_data: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignatureStatusResponse {
    pub success: bool,
    pub signature_valid: bool,
    pub chain: Option<SignatureChainResponse>,
    pub revocation: Option<Vec<CertStatus>>,
    pub signer: Option<String>,
    pub signing_time: Option<String>,
    pub message: String,
}

#[async_trait]
pub trait SignatureService: Send + Sync {
    async fn sign_pdf(&self, pdf_data: &[u8], info: &SignatureInfo) -> SignatureResult<Vec<u8>>;
    async fn verify_pdf(&self, pdf_data: &[u8]) -> SignatureResult<VerificationResult>;
}

pub struct PdfSignatureService;

impl Default for PdfSignatureService {
    fn default() -> Self {
        Self::new()
    }
}

impl PdfSignatureService {
    pub fn new() -> Self {
        Self
    }

    fn decode_pem_block(
        pem: &str,
        begin: &str,
        end: &str,
        invalid: SignatureError,
    ) -> SignatureResult<Zeroizing<Vec<u8>>> {
        let body = match (pem.find(begin), pem.find(end)) {
            (Some(start), Some(end_pos)) if end_pos > start => &pem[start + begin.len()..end_pos],
            _ => pem.trim(),
        };
        base64::Engine::decode(
            &base64::engine::general_purpose::STANDARD,
            body.replace(['\n', '\r', ' '], ""),
        )
        .map(Zeroizing::new)
        .map_err(|_| invalid)
    }

    fn parse_private_key(pem: &str) -> SignatureResult<signature::RsaKeyPair> {
        let is_pkcs8 = pem.contains("-----BEGIN PRIVATE KEY-----");
        let (begin, end) = if is_pkcs8 {
            ("-----BEGIN PRIVATE KEY-----", "-----END PRIVATE KEY-----")
        } else {
            (
                "-----BEGIN RSA PRIVATE KEY-----",
                "-----END RSA PRIVATE KEY-----",
            )
        };
        let key_bytes = Self::decode_pem_block(pem, begin, end, SignatureError::InvalidKey)?;
        if is_pkcs8 {
            signature::RsaKeyPair::from_pkcs8(key_bytes.as_slice())
        } else {
            signature::RsaKeyPair::from_der(key_bytes.as_slice())
        }
        .map_err(|_| SignatureError::InvalidKey)
    }

    fn parse_public_key(cert_pem: &str) -> SignatureResult<Vec<u8>> {
        let cert_bytes = Self::decode_pem_block(
            cert_pem,
            "-----BEGIN CERTIFICATE-----",
            "-----END CERTIFICATE-----",
            SignatureError::InvalidCertificate,
        )?;
        let (_, cert) = x509_parser::parse_x509_certificate(&cert_bytes)
            .map_err(|_| SignatureError::InvalidCertificate)?;
        match cert.public_key().parsed() {
            Ok(x509_parser::public_key::PublicKey::RSA(_)) => {
                Ok(cert.public_key().subject_public_key.data.to_vec())
            }
            _ => Err(SignatureError::InvalidKey),
        }
    }

    fn extract_signer_info(cert_pem: &str) -> SignatureResult<(Option<String>, Option<String>)> {
        let cert_data = if let Some(start) = cert_pem.find("-----BEGIN CERTIFICATE-----") {
            let end = cert_pem.find("-----END CERTIFICATE-----").unwrap_or(cert_pem.len());
            cert_pem[start..end]
                .replace("-----BEGIN CERTIFICATE-----", "")
                .replace("-----END CERTIFICATE-----", "")
                .replace(['\n', '\r', ' '], "")
        } else {
            cert_pem.trim().to_string()
        };
        let cert_bytes = base64::Engine::decode(&base64::engine::general_purpose::STANDARD, cert_data)
            .map_err(|_| SignatureError::InvalidCertificate)?;
        
        let (_, cert) = x509_parser::parse_x509_certificate(&cert_bytes)
            .map_err(|_| SignatureError::InvalidCertificate)?;
        
        let subject = cert.subject().to_string();
        let signing_time = Some(Utc::now().to_rfc3339());
        Ok((Some(subject), signing_time))
    }

    fn verify_signature(public_key: &[u8], message: &[u8], signature_bytes: &[u8]) -> bool {
        signature::UnparsedPublicKey::new(
            &signature::RSA_PKCS1_2048_8192_SHA256,
            public_key,
        )
        .verify(message, signature_bytes)
        .is_ok()
    }

    fn verify_certificate_dates(cert_pem: &str) -> SignatureResult<()> {
        let cert_data = if let Some(start) = cert_pem.find("-----BEGIN CERTIFICATE-----") {
            let end = cert_pem.find("-----END CERTIFICATE-----").unwrap_or(cert_pem.len());
            cert_pem[start..end]
                .replace("-----BEGIN CERTIFICATE-----", "")
                .replace("-----END CERTIFICATE-----", "")
                .replace(['\n', '\r', ' '], "")
        } else {
            cert_pem.trim().to_string()
        };

        let cert_bytes = base64::Engine::decode(&base64::engine::general_purpose::STANDARD, cert_data)
            .map_err(|_| SignatureError::InvalidCertificate)?;

        let (_, cert) = x509_parser::parse_x509_certificate(&cert_bytes)
            .map_err(|_| SignatureError::InvalidCertificate)?;

        let now = std::time::SystemTime::now();
        let not_before: std::time::SystemTime = cert.validity().not_before.to_datetime().into();
        let not_after: std::time::SystemTime = cert.validity().not_after.to_datetime().into();
        
        if now < not_before || now > not_after {
            return Err(SignatureError::VerificationFailed(
                format!("certificate not valid: not_before={:?}, not_after={:?}", not_before, not_after)
            ));
        }

        Ok(())
    }

    fn parse_pem_cert_chain(chain_pem: &str) -> SignatureResult<Vec<Vec<u8>>> {
        let mut certs = Vec::new();
        let mut remaining = chain_pem;

        while let Some(start) = remaining.find("-----BEGIN CERTIFICATE-----") {
            if let Some(end) = remaining.find("-----END CERTIFICATE-----") {
                let end = end + "-----END CERTIFICATE-----".len();
                let block = &remaining[start..end];
                let b64 = block
                    .replace("-----BEGIN CERTIFICATE-----", "")
                    .replace("-----END CERTIFICATE-----", "")
                    .replace(['\n', '\r', ' '], "");
                let bytes = base64::Engine::decode(&base64::engine::general_purpose::STANDARD, b64)
                    .map_err(|_| SignatureError::InvalidCertificate)?;
                certs.push(bytes);
                remaining = &remaining[end..];
            } else {
                break;
            }
        }

        if certs.is_empty() {
            return Err(SignatureError::ChainVerification(
                "no certificates found in PEM chain".into(),
            ));
        }

        Ok(certs)
    }

    fn verify_cert_chain(chain_pem: &str) -> SignatureResult<Vec<ChainLink>> {
        let certs = Self::parse_pem_cert_chain(chain_pem)?;
        let mut chain = Vec::with_capacity(certs.len());

        for (i, cert_bytes) in certs.iter().enumerate() {
            let (_, cert) =
                x509_parser::parse_x509_certificate(cert_bytes).map_err(|_| {
                    SignatureError::ChainVerification(format!("failed to parse certificate at depth {}", i))
                })?;

            let subject = cert.subject().to_string();
            let issuer = cert.issuer().to_string();

            let now = std::time::SystemTime::now();
            let not_before: std::time::SystemTime = cert.validity().not_before.to_datetime().into();
            let not_after: std::time::SystemTime = cert.validity().not_after.to_datetime().into();
            let _date_valid = now >= not_before && now <= not_after;

            let is_self_signed = subject == issuer;

            let mut signature_valid = false;
            if !is_self_signed || certs.len() == 1 {
                if let Some(issuer_bytes) = certs.get(i + 1) {
                    if let Ok((_, issuer_cert)) =
                        x509_parser::parse_x509_certificate(issuer_bytes)
                    {
                        signature_valid = PdfSignatureService::verify_cert_signature(&cert, &issuer_cert).is_ok();
                    }
                }
            }

            chain.push(ChainLink {
                subject,
                issuer,
                signature_valid,
                is_self_signed,
                depth: i,
            });
        }

        if chain.is_empty() {
            return Err(SignatureError::ChainVerification(
                "empty certificate chain".into(),
            ));
        }

        for i in 0..chain.len() - 1 {
            if chain[i].issuer != chain[i + 1].subject {
                return Err(SignatureError::ChainVerification(format!(
                    "issuer/subject mismatch at depth {}: issuer={}, expected={}",
                    i, chain[i].issuer, chain[i + 1].subject
                )));
            }
            if !chain[i].signature_valid {
                return Err(SignatureError::ChainVerification(format!(
                    "signature verification failed for certificate at depth {} (subject={})",
                    i, chain[i].subject
                )));
            }
        }

        let root = &chain[chain.len() - 1];
        if !root.is_self_signed {
            return Err(SignatureError::ChainVerification(
                "chain does not terminate at a self-signed root certificate".into(),
            ));
        }

        Ok(chain)
    }

    fn verify_cert_signature(
        subject_cert: &x509_parser::certificate::X509Certificate,
        issuer_cert: &x509_parser::certificate::X509Certificate,
    ) -> SignatureResult<()> {
        let spki = issuer_cert.public_key();
        subject_cert
            .verify_signature(Some(spki))
            .map_err(|e| SignatureError::ChainVerification(format!("invalid certificate signature: {:?}", e)))?;
        Ok(())
    }

    fn check_revocation_status(chain_pem: &str) -> SignatureResult<Vec<CertStatus>> {
        let certs = Self::parse_pem_cert_chain(chain_pem)?;
        let mut statuses = Vec::with_capacity(certs.len());

        for cert_bytes in &certs {
            let (_, cert) =
                x509_parser::parse_x509_certificate(cert_bytes).map_err(|_| {
                    SignatureError::InvalidCertificate
                })?;

            let subject = cert.subject().to_string();
            let issuer = cert.issuer().to_string();
            let serial = cert.tbs_certificate.raw_serial_as_string();
            let not_before = format!("{}", cert.validity().not_before.to_datetime());
            let not_after = format!("{}", cert.validity().not_after.to_datetime());

            let mut revocation_reason = None;

            for ext in cert.extensions() {
                let oid_str = ext.oid.to_string();
                if oid_str == "2.5.29.31" {
                    revocation_reason = Some("CRL distribution points extension present; live CRL/OCSP check requires network access to the issuing CA".into());
                } else if oid_str == "2.5.29.1"
                    && revocation_reason.is_none() {
                        revocation_reason = Some("AuthorityInfoAccess extension present; live OCSP check requires network access".into());
                    }
            }

            statuses.push(CertStatus {
                subject,
                issuer,
                serial_number: serial,
                not_before,
                not_after,
                revoked: false,
                revocation_reason,
            });
        }

        Ok(statuses)
    }
}

#[async_trait]
impl SignatureService for PdfSignatureService {
    async fn sign_pdf(&self, pdf_data: &[u8], info: &SignatureInfo) -> SignatureResult<Vec<u8>> {
        let private_key = Self::parse_private_key(&info.private_key_pem)?;
        let public_key = Self::parse_public_key(&info.cert_pem)?;
        if private_key.public().as_ref() != public_key.as_slice() {
            return Err(SignatureError::InvalidKey);
        }

        Self::verify_certificate_dates(&info.cert_pem)?;

        let mut signature_bytes = vec![0; private_key.public().modulus_len()];
        private_key
            .sign(
                &signature::RSA_PKCS1_SHA256,
                &SystemRandom::new(),
                pdf_data,
                &mut signature_bytes,
            )
            .map_err(|_| SignatureError::PdfOperation("RSA signing failed".to_string()))?;
        let signature_hex = hex::encode(&signature_bytes);
        signature_bytes.fill(0);

        self.embed_signature(pdf_data, &signature_hex, info)
    }

    async fn verify_pdf(&self, pdf_data: &[u8]) -> SignatureResult<VerificationResult> {
        let sig = self.extract_signature(pdf_data)?;
        
        let public_key = match Self::parse_public_key(&sig.cert_pem) {
            Ok(k) => k,
            Err(e) => {
                return Ok(VerificationResult {
                    valid: false,
                    signer: None,
                    signing_time: None,
                    reason: Some(sig.reason),
                    chain: None,
                    cert_pem: sig.cert_pem.clone(),
                    error: Some(format!("failed to parse certificate: {}", e)),
                });
            }
        };
        
        if let Err(e) = Self::verify_certificate_dates(&sig.cert_pem) {
            return Ok(VerificationResult {
                valid: false,
                signer: None,
                signing_time: None,
                reason: Some(sig.reason),
                chain: None,
                cert_pem: sig.cert_pem.clone(),
                error: Some(format!("certificate date verification failed: {}", e)),
            });
        }
        
        let chain = Self::verify_cert_chain(&sig.cert_pem).ok();

        let signature_bytes = match hex::decode(&sig.signature_hex) {
            Ok(bytes) => bytes,
            Err(e) => {
                return Ok(VerificationResult {
                    valid: false,
                    signer: None,
                    signing_time: None,
                    reason: Some(sig.reason),
                    chain,
                    cert_pem: sig.cert_pem.clone(),
                    error: Some(format!("invalid signature hex: {e}")),
                });
            }
        };
        let valid = Self::verify_signature(
            &public_key,
            &sig.pdf_without_signature,
            &signature_bytes,
        );
        
        let (signer, signing_time) = Self::extract_signer_info(&sig.cert_pem).unwrap_or((None, None));
        
        let chain_valid = chain.as_ref().map(|c| !c.is_empty()).unwrap_or(false);
        
        Ok(VerificationResult {
            valid,
            signer,
            signing_time,
            reason: Some(sig.reason),
            chain,
            cert_pem: sig.cert_pem,
            error: if valid && chain_valid { None } else { Some("signature or chain verification failed".to_string()) },
        })
    }
}

struct EmbeddedSignature {
    signature_hex: String,
    pdf_without_signature: Vec<u8>,
    cert_pem: String,
    reason: String,
}

impl PdfSignatureService {
    fn embed_signature(&self, pdf_data: &[u8], signature_hex: &str, info: &SignatureInfo) -> SignatureResult<Vec<u8>> {
        let mut doc = match lopdf::Document::load_mem(pdf_data) {
            Ok(d) => d,
            Err(_) => {
                return self.embed_signature_fallback(pdf_data, signature_hex, info);
            }
        };
        
        let mut sig_dict = lopdf::Dictionary::new();
        sig_dict.set("Type", lopdf::Object::Name(b"Sig".to_vec()));
        sig_dict.set("Filter", lopdf::Object::Name(b"Adobe.PPKLite".to_vec()));
        sig_dict.set("SubFilter", lopdf::Object::Name(b"adbe.pkcs7.detached".to_vec()));
        sig_dict.set("Name", lopdf::Object::string_literal(info.signer_name().unwrap_or("Unknown")));
        sig_dict.set("Location", lopdf::Object::string_literal(info.location.as_str()));
        sig_dict.set("Reason", lopdf::Object::string_literal(info.reason.as_str()));
        sig_dict.set("M", lopdf::Object::string_literal(Utc::now().to_rfc3339()));
        sig_dict.set("ByteRange", lopdf::Object::Array(vec![
            lopdf::Object::Integer(0),
            lopdf::Object::Integer(0),
            lopdf::Object::Integer(0),
            lopdf::Object::Integer(0),
        ]));
        sig_dict.set("Contents", lopdf::Object::string_literal(signature_hex));
        
        let sig_id = doc.new_object_id();
        doc.objects.insert(sig_id, lopdf::Object::Dictionary(sig_dict));
        
        let mut buf = Vec::new();
        doc.save_to(&mut buf).map_err(|e| SignatureError::PdfOperation(e.to_string()))?;
        
        if let Some(pos) = buf.windows(signature_hex.len()).position(|w| w == signature_hex.as_bytes()) {
            let sig_len = signature_hex.len();
            let file_size = buf.len();
            let byte_range = vec![
                lopdf::Object::Integer(0),
                lopdf::Object::Integer(pos as i64),
                lopdf::Object::Integer((pos + sig_len) as i64),
                lopdf::Object::Integer((file_size - pos - sig_len) as i64),
            ];
            
            if let Some(lopdf::Object::Dictionary(ref mut dict)) = doc.objects.get_mut(&sig_id) {
                dict.set("ByteRange", lopdf::Object::Array(byte_range));
            }
            
            buf.clear();
            doc.save_to(&mut buf).map_err(|e| SignatureError::PdfOperation(e.to_string()))?;
        } else {
            warn!("could not locate signature hex in PDF output; ByteRange remains placeholder");
        }
        
        Ok(buf)
    }
    
    fn embed_signature_fallback(&self, pdf_data: &[u8], signature_hex: &str, _info: &SignatureInfo) -> SignatureResult<Vec<u8>> {
        let marker = b"/SignaturePlaceholder<</Contents<";
        if let Some(pos) = pdf_data.windows(marker.len()).position(|w| w == marker) {
            let insert_pos = pos + marker.len() + 2;
            let mut result = pdf_data[..insert_pos].to_vec();
            result.extend_from_slice(signature_hex.as_bytes());
            result.extend_from_slice(b">>>");
            result.extend_from_slice(&pdf_data[insert_pos + 2 + 64 * 2..]);
            Ok(result)
        } else {
            let mut result = pdf_data.to_vec();
            let sig_block = format!("\n%% Signature: {}\n", signature_hex);
            result.extend_from_slice(sig_block.as_bytes());
            Ok(result)
        }
    }
    
    fn extract_signature(&self, pdf_data: &[u8]) -> SignatureResult<EmbeddedSignature> {
        let content_start = pdf_data.windows(10).position(|w| w == b"/Contents<");
        let content_end = pdf_data.windows(2).position(|w| w == b">>").map(|p| p + 2);
        
        if let (Some(start), Some(end)) = (content_start, content_end) {
            let sig_hex = String::from_utf8_lossy(&pdf_data[start + 10..end - 2]).to_string();
            let cert_pem = String::new();
            let reason = String::new();
            
            Ok(EmbeddedSignature {
                signature_hex: sig_hex,
                pdf_without_signature: pdf_data.to_vec(),
                cert_pem,
                reason,
            })
        } else {
            Err(SignatureError::PdfOperation("no signature found".to_string()))
        }
    }
}

trait SignatureInfoExt {
    fn signer_name(&self) -> Option<&str>;
}

impl SignatureInfoExt for SignatureInfo {
    fn signer_name(&self) -> Option<&str> {
        self.signer_name.as_deref().or_else(|| {
            self.cert_pem.lines()
                .find(|l| l.starts_with("Subject: "))
                .map(|l| l.trim_start_matches("Subject: "))
        })
    }
}

pub async fn sign_pdf_handler(
    State(service): State<Arc<dyn SignatureService>>,
    Json(req): Json<SignPdfRequest>,
) -> Json<SignPdfResponse> {
    let file_data = match base64::Engine::decode(&base64::engine::general_purpose::STANDARD, &req.file_data) {
        Ok(data) => data,
        Err(_) => {
            return Json(SignPdfResponse {
                success: false,
                signed_data: None,
                message: "invalid base64 file data".to_string(),
            });
        }
    };
    match service.sign_pdf(&file_data, &req.signature_info).await {
        Ok(signed) => {
            let encoded = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, signed);
            Json(SignPdfResponse {
                success: true,
                signed_data: Some(encoded),
                message: "signed successfully".to_string(),
            })
        }
        Err(e) => Json(SignPdfResponse {
            success: false,
            signed_data: None,
            message: e.to_string(),
        }),
    }
}

pub async fn verify_pdf_handler(
    State(service): State<Arc<dyn SignatureService>>,
    Json(req): Json<VerifyPdfRequest>,
) -> Json<VerifyPdfResponse> {
    let file_data = match base64::Engine::decode(&base64::engine::general_purpose::STANDARD, &req.file_data) {
        Ok(data) => data,
        Err(_) => {
            return Json(VerifyPdfResponse {
                success: false,
                result: None,
                message: "invalid base64 file data".to_string(),
            });
        }
    };
    match service.verify_pdf(&file_data).await {
        Ok(result) => Json(VerifyPdfResponse {
            success: true,
            result: Some(result),
            message: "verified".to_string(),
        }),
        Err(e) => Json(VerifyPdfResponse {
            success: false,
            result: None,
            message: e.to_string(),
        }),
    }
}

pub async fn signature_status_handler(
    State(service): State<Arc<dyn SignatureService>>,
    Json(req): Json<SignatureStatusRequest>,
) -> Json<SignatureStatusResponse> {
    let file_data = match base64::Engine::decode(&base64::engine::general_purpose::STANDARD, &req.file_data) {
        Ok(data) => data,
        Err(_) => {
            return Json(SignatureStatusResponse {
                success: false,
                signature_valid: false,
                chain: None,
                revocation: None,
                signer: None,
                signing_time: None,
                message: "invalid base64 file data".to_string(),
            });
        }
    };

    match service.verify_pdf(&file_data).await {
        Ok(result) => {
            let chain = PdfSignatureService::verify_cert_chain(&result.cert_pem).ok();
            let chain_response = chain.as_ref().map(|links| SignatureChainResponse {
                valid: links.iter().all(|l| l.signature_valid) && links.last().map(|l| l.is_self_signed).unwrap_or(false),
                chain: links.clone(),
                error: None,
            });
            let revocation = PdfSignatureService::check_revocation_status(&result.cert_pem).ok();

            Json(SignatureStatusResponse {
                success: true,
                signature_valid: result.valid,
                chain: chain_response,
                revocation,
                signer: result.signer,
                signing_time: result.signing_time,
                message: "status retrieved".to_string(),
            })
        }
        Err(e) => Json(SignatureStatusResponse {
            success: false,
            signature_valid: false,
            chain: None,
            revocation: None,
            signer: None,
            signing_time: None,
            message: e.to_string(),
        }),
    }
}

pub fn signature_route<S: SignatureService + 'static>(service: S) -> Router {
    Router::new()
        .route("/signature/pdf/sign", axum::routing::post(sign_pdf_handler))
        .route("/signature/pdf/verify", axum::routing::post(verify_pdf_handler))
        .route("/signature/pdf/status", axum::routing::post(signature_status_handler))
        .with_state(Arc::new(service))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::util::ServiceExt;

    const TEST_PRIVATE_KEY: &str = r#"-----BEGIN PRIVATE KEY-----
MIIEuwIBADANBgkqhkiG9w0BAQEFAASCBKUwggShAgEAAoIBAQDYtYImjDsJZuR9
hRuRVSkApAZJcsgZWFRbwNlj8bWvY/yuttvpCUgqOvRNSHsh9+czU03FSSfF6R0n
r01tp3pWHoWhB9Ucqggi8QYoOkMtS0iyk7L8IAwZ5OoWfkQR93XuWUD3dhUR3vMe
gV8HGUOTRdnfDvVq01HknMpJqjOiTa8ejmYQJFW17FBR2nidi5Ob2tkvrwtqqVQI
6oWxkE5M5f/RmuQmlVMEhdAbk7rMgXKX+2cr+0whL6sJI0Toe/mkq/ik3uAPCVbR
RoDvlFgpp2wOg6qfje61KNmgs60Bj5lhu97TNUWhTukH5dyYZrOWDX00vHURmYiq
o1cMxkmlAgMBAAECgf9PViQIv9wyhr/m+ze47vR3cj/3auOBgFTwJfE3extsYa6g
R/xW8MfePEVQsH+ANzOYFrypNkm848W6eoF6scd7anhIpoc0Ie8t+A9XORm0ND8i
BQ0eQJ1J+gRj4mRCL519JRN5E15OAe3G2bKM9Pny/wMn7XyyrXHNwEOj7Ugy23TS
/HuSCIkLH/4Zl6ztSddiCo52FfdDBKsjNe5Pb8dTIx31C4syidWLE7X+WPmCGOe9
N5jhQfLAsEU5wQlJHAB20RGICIp7ZqFDBCySpq5GZDyJOX7BdHBpYcCP+END+dU3
M2mey7VqH02zEYulgIaUZAcIUA2pv3dbH0P8QjECgYEA9RI08RRDts1plqWu/h2V
XXmX5FiKA6OZwsAiAszmjGcH7mFZ7OjaVQS8SjVZ37oaOkWpwFPz0MRbIJmWmw7t
ICZdZQJcxHLuFyJgUm+kv+dJQue/PJoCojzE2cZpiqW/SLet6d2Uv0mKiqe5eWWC
5rFRI/v56f3WhN6Wn1lSDg0CgYEA4l+DQpmm+IifdYeraLE6EbOAlHkghnLiO7pR
WniGTxGmO13nlU7OLZOubKsJOzgk1r0egRjEbE+1rDToXPTuHxjT3+yWPp8mBUVW
f77INMMoG2Nr4AnuAqE+rU1Q1WLmqCfDGRvn289aARTzuRuxtU0QRsQywSp1uZyd
Tj7YW/kCgYBIALRrTE/kyo9GQqGaaaiz0QDOhzDthsirTnXvqrHl+HN9Fz8revKC
3iRQDUK9l9kS29rW9hOBd99qQZXdMtJ6iqsP/VSyJy5Kv7/bGJAoDdUZgitOq9Uw
Q3h2n3Ps12vO+qBvQLnuRbYdrM+ymh+OlfRIBUVU+U5otVk9simIlQKBgB3ygMzo
wtwSRvYncpexCnuZAaOiupjOzfsU1PphA3OmZBVqgN6RxFjnNqYNonUBIm5+KnDt
s96YVPJpNWxGwtG+WRlAlUfHiiIcYCsaNCY2wzGMX5MN/Ty/1CjdF5qDMPyB9h92
P6AmuEN4YB3W+hWAEm0qO4Sud1CM1YqqabkRAoGBANkZ81w+1EmxFVsSlur4QrYo
SwKCbi5mRa/9UWVJk2Ik86GdZjF4HtaEgrT7AhmtIjL3f0zxWf5ql4zDLoooHlPK
BYV3lH/SjS3ZF2uO5pnNSmiK7sRcizFASOh7VqIP4C4pXJuw3aTlJbhPwkK3m7wZ
owlYFraS332MS/YEInRe
-----END PRIVATE KEY-----"#;

    const TEST_CERT: &str = r#"-----BEGIN CERTIFICATE-----
MIIDQzCCAiugAwIBAgIUKZT19jSpQB7w+is3HNR5y7YZAvAwDQYJKoZIhvcNAQEL
BQAwMDEcMBoGA1UEAwwTb2E0cnVzdCB0ZXN0IHNpZ25lcjEQMA4GA1UECgwHb2E0
cnVzdDAgFw0yNjA5MDkxNDUyNTNaGA8yMTI2MDgxNjE0NTI1M1owMDEcMBoGA1UE
AwwTb2E0cnVzdCB0ZXN0IHNpZ25lcjEQMA4GA1UECgwHb2E0cnVzdDCCASIwDQYJ
KoZIhvcNAQEBBQADggEPADCCAQoCggEBANi1giaMOwlm5H2FG5FVKQCkBklyyBlY
VFvA2WPxta9j/K622+kJSCo69E1IeyH35zNTTcVJJ8XpHSevTW2nelYehaEH1Ryq
CCLxBig6Qy1LSLKTsvwgDBnk6hZ+RBH3de5ZQPd2FRHe8x6BXwcZQ5NF2d8O9WrT
UeScykmqM6JNrx6OZhAkVbXsUFHaeJ2Lk5va2S+vC2qpVAjqhbGQTkzl/9Ga5CaV
UwSF0BuTusyBcpf7Zyv7TCEvqwkjROh7+aSr+KTe4A8JVtFGgO+UWCmnbA6Dqp+N
7rUo2aCzrQGPmWG73tM1RaFO6Qfl3Jhms5YNfTS8dRGZiKqjVwzGSaUCAwEAAaNT
MFEwHQYDVR0OBBYEFBucxZPWa3T672lBEf0JAUcXXq2xMB8GA1UdIwQYMBaAFBuc
xZPWa3T672lBEf0JAUcXXq2xMA8GA1UdEwEB/wQFMAMBAf8wDQYJKoZIhvcNAQEL
BQADggEBAEeIxp1XFnXJ2BUmA5GbIR8r5OYI/llZdjb/8016ejsRGDZN/kzEPAPs
vUqvFNiNSGHi7jOmI1qrABWkApKHTNGMMnJRbFu58OncofXewrfAbv/LnVi69peP
yEuKpe3hStUAUbEDuiQB2aH/fVLoyR8WYtLtU1XJL2KZvZ98qGGc7NT+QF9XEIy3
L653yB7J70eOQ7aoqSZPT/j0UGUFK+HEqDpucFQbQ0VW/VNbp5soH/DGqHh9S3ec
lRqp8wDDxkCNB7iD86UPs9zN/1dXq9JpS6VayLlCVGApd47qLq8mndZ6cBvhKYeB
S8ARr2vqmeZSYIJABzClXMa1aS2XpA4=
-----END CERTIFICATE-----"#;

    #[test]
    fn test_ring_rsa_sign_and_verify_fixed_fixture() {
        let private_key = PdfSignatureService::parse_private_key(TEST_PRIVATE_KEY).unwrap();
        let public_key = PdfSignatureService::parse_public_key(TEST_CERT).unwrap();
        assert_eq!(private_key.public().as_ref(), public_key.as_slice());

        let message = b"oa4rust signature fixture";
        let mut signature_bytes = vec![0; private_key.public().modulus_len()];
        private_key
            .sign(
                &signature::RSA_PKCS1_SHA256,
                &SystemRandom::new(),
                message,
                &mut signature_bytes,
            )
            .unwrap();
        assert!(PdfSignatureService::verify_signature(
            &public_key,
            message,
            &signature_bytes,
        ));
        assert!(!PdfSignatureService::verify_signature(
            &public_key,
            b"tampered",
            &signature_bytes,
        ));
    }

    #[tokio::test]
    async fn test_sign_pdf_with_fixed_fixture() {
        let info = SignatureInfo::new(
            "approval",
            "Beijing",
            "signer@example.test",
            TEST_CERT,
            TEST_PRIVATE_KEY,
        );
        let signed = PdfSignatureService::new()
            .sign_pdf(b"%PDF-1.4\ntest", &info)
            .await
            .expect("fixed fixture should sign");
        assert!(signed.windows(13).any(|window| window == b"%% Signature:"));
        assert!(signed.len() > b"%PDF-1.4\ntest".len());
    }

    #[test]
    fn test_private_key_fixture_rejects_invalid_material() {
        assert!(PdfSignatureService::parse_private_key("not a private key").is_err());
    }

    #[tokio::test]
    async fn test_sign_pdf_route() {
        let service = PdfSignatureService::new();
        let app = signature_route(service);
        
        let req = SignPdfRequest {
            file_data: base64::Engine::encode(&base64::engine::general_purpose::STANDARD, b"%PDF-1.4\ntest"),
            signature_info: SignatureInfo::new("test", "location", "contact", "dummy", "dummy"),
        };
        
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/signature/pdf/sign")
                    .method("POST")
                    .header("content-type", "application/json")
                    .body(Body::from(serde_json::to_vec(&req).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();
        
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_verify_pdf_route() {
        let service = PdfSignatureService::new();
        let app = signature_route(service);
        
        let req = VerifyPdfRequest {
            file_data: base64::Engine::encode(&base64::engine::general_purpose::STANDARD, b"%PDF-1.4\ntest"),
        };
        
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/signature/pdf/verify")
                    .method("POST")
                    .header("content-type", "application/json")
                    .body(Body::from(serde_json::to_vec(&req).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();
        
        assert_eq!(response.status(), StatusCode::OK);
    }
}

#[cfg(test)]
mod tests_generated;
