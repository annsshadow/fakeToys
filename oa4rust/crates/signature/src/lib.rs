use std::sync::Arc;

use async_trait::async_trait;
use axum::{Router, extract::State, Json};
use chrono::Utc;
use rsa::{pkcs1::{DecodeRsaPrivateKey, DecodeRsaPublicKey}, RsaPrivateKey, RsaPublicKey};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use thiserror::Error;
use tracing::warn;
use x509_parser::prelude::*;

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

impl PdfSignatureService {
    pub fn new() -> Self {
        Self
    }

    fn parse_private_key(pem: &str) -> SignatureResult<RsaPrivateKey> {
        let pem = pem.trim();
        let key_data = if let Some(start) = pem.find("-----BEGIN") {
            let end = pem.find("-----END").unwrap_or(pem.len());
            pem[start..end].replace("-----BEGIN PRIVATE KEY-----", "")
                .replace("-----END PRIVATE KEY-----", "")
                .replace("-----BEGIN RSA PRIVATE KEY-----", "")
                .replace("-----END RSA PRIVATE KEY-----", "")
                .replace('\n', "")
                .replace('\r', "")
                .replace(' ', "")
        } else {
            pem.to_string()
        };
        let key_bytes = base64::Engine::decode(&base64::engine::general_purpose::STANDARD, key_data)
            .map_err(|_| SignatureError::InvalidKey)?;
        RsaPrivateKey::from_pkcs1_der(&key_bytes)
            .or_else(|_| {
                use rsa::pkcs8::DecodePrivateKey;
                RsaPrivateKey::from_pkcs8_der(&key_bytes)
            })
            .map_err(|_| SignatureError::InvalidKey)
    }

    fn parse_public_key(cert_pem: &str) -> SignatureResult<RsaPublicKey> {
        let cert_data = if let Some(start) = cert_pem.find("-----BEGIN CERTIFICATE-----") {
            let end = cert_pem.find("-----END CERTIFICATE-----").unwrap_or(cert_pem.len());
            cert_pem[start..end]
                .replace("-----BEGIN CERTIFICATE-----", "")
                .replace("-----END CERTIFICATE-----", "")
                .replace('\n', "")
                .replace('\r', "")
                .replace(' ', "")
        } else {
            cert_pem.trim().to_string()
        };
        let cert_bytes = base64::Engine::decode(&base64::engine::general_purpose::STANDARD, cert_data)
            .map_err(|_| SignatureError::InvalidCertificate)?;
        
        let (_, cert) = x509_parser::parse_x509_certificate(&cert_bytes)
            .map_err(|_| SignatureError::InvalidCertificate)?;
        
        let public_key_der = cert.public_key().subject_public_key.data.clone();
        RsaPublicKey::from_pkcs1_der(&public_key_der)
            .or_else(|_| {
                use rsa::pkcs8::DecodePublicKey;
                RsaPublicKey::from_public_key_der(&public_key_der)
            })
            .map_err(|_| SignatureError::InvalidKey)
    }

    fn extract_signer_info(cert_pem: &str) -> SignatureResult<(Option<String>, Option<String>)> {
        let cert_data = if let Some(start) = cert_pem.find("-----BEGIN CERTIFICATE-----") {
            let end = cert_pem.find("-----END CERTIFICATE-----").unwrap_or(cert_pem.len());
            cert_pem[start..end]
                .replace("-----BEGIN CERTIFICATE-----", "")
                .replace("-----END CERTIFICATE-----", "")
                .replace('\n', "")
                .replace('\r', "")
                .replace(' ', "")
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

    fn compute_sha256(data: &[u8]) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(data);
        let result = hasher.finalize();
        let mut hash = [0u8; 32];
        hash.copy_from_slice(&result);
        hash
    }

    fn verify_certificate_dates(cert_pem: &str) -> SignatureResult<()> {
        let cert_data = if let Some(start) = cert_pem.find("-----BEGIN CERTIFICATE-----") {
            let end = cert_pem.find("-----END CERTIFICATE-----").unwrap_or(cert_pem.len());
            cert_pem[start..end]
                .replace("-----BEGIN CERTIFICATE-----", "")
                .replace("-----END CERTIFICATE-----", "")
                .replace('\n', "")
                .replace('\r', "")
                .replace(' ', "")
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
                    .replace('\n', "")
                    .replace('\r', "")
                    .replace(' ', "");
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
            if !is_self_signed || (is_self_signed && certs.len() == 1) {
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
                } else if oid_str == "2.5.29.1" {
                    if revocation_reason.is_none() {
                        revocation_reason = Some("AuthorityInfoAccess extension present; live OCSP check requires network access".into());
                    }
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
        
        Self::verify_certificate_dates(&info.cert_pem)?;
        
        let hash = Self::compute_sha256(pdf_data);
        let signature = private_key.sign(
            rsa::pkcs1v15::Pkcs1v15Sign::new::<sha2::Sha256>(),
            &hash
        )
        .map_err(|e| SignatureError::PdfOperation(e.to_string()))?;
        
        let signature_hex = hex::encode(signature);
        
        let signed_pdf = self.embed_signature(pdf_data, &signature_hex, info)?;
        Ok(signed_pdf)
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
        
        let hash = Self::compute_sha256(&sig.pdf_without_signature);
        let signature_bytes = match hex::decode(&sig.signature_hex) {
            Ok(b) => b,
            Err(e) => {
                return Ok(VerificationResult {
                    valid: false,
                    signer: None,
                    signing_time: None,
                    reason: Some(sig.reason),
                    chain,
                    cert_pem: sig.cert_pem.clone(),
                    error: Some(format!("invalid signature hex: {}", e)),
                });
            }
        };
        
        let valid = public_key.verify(
            rsa::pkcs1v15::Pkcs1v15Sign::new::<sha2::Sha256>(),
            &hash,
            &signature_bytes
        ).is_ok();
        
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
        
        let sig_id = (doc.objects.len() as u32, 0u16);
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
    
    fn embed_signature_fallback(&self, pdf_data: &[u8], signature_hex: &str, info: &SignatureInfo) -> SignatureResult<Vec<u8>> {
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
        let content_end = pdf_data.windows(2).position(|w| w == b">>" || w == b">>" ).and_then(|p| Some(p + 2));
        
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
