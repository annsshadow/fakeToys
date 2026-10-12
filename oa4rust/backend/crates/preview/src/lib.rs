// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::path::PathBuf;
use std::sync::Arc;

use async_trait::async_trait;
use axum::{extract::State, Json, Router};
use base64::{engine::general_purpose, Engine as _};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::info;

#[derive(Debug, Error)]
pub enum PreviewError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("unsupported file type: {0}")]
    UnsupportedType(String),
    #[error("conversion failed: {0}")]
    ConversionFailed(String),
    #[error("upload failed: {0}")]
    UploadFailed(String),
    #[error("download failed: {0}")]
    DownloadFailed(String),
    #[error("serialization error: {0}")]
    Serialize(#[from] serde_json::Error),
}

pub type PreviewResult<T> = Result<T, PreviewError>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreviewRequest {
    pub file_name: String,
    pub file_data: String,
    pub file_type: Option<String>,
    pub target_format: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreviewResponse {
    pub success: bool,
    pub preview_url: Option<String>,
    pub download_url: Option<String>,
    pub file_url: Option<String>,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadedFile {
    pub file_url: String,
    pub file_name: String,
    pub file_type: String,
    pub size: u64,
}

#[async_trait]
pub trait PreviewService: Send + Sync {
    async fn preview(&self, file_data: &[u8], file_name: &str) -> PreviewResult<PreviewResponse>;
    async fn convert(
        &self,
        file_data: &[u8],
        file_name: &str,
        target_format: &str,
    ) -> PreviewResult<Vec<u8>>;
    async fn upload(&self, file_data: &[u8], file_name: &str) -> PreviewResult<UploadedFile>;
    async fn download(&self, file_url: &str) -> PreviewResult<Vec<u8>>;
}

#[derive(Clone)]
pub struct OnlyOfficePreview {
    client: Client,
    doc_server_url: String,
}

impl OnlyOfficePreview {
    pub fn new(doc_server_url: impl Into<String>) -> Self {
        Self {
            client: Client::new(),
            doc_server_url: doc_server_url.into(),
        }
    }

    fn detect_mime_type(file_name: &str) -> &'static str {
        let ext = PathBuf::from(file_name)
            .extension()
            .and_then(|s| s.to_str())
            .map(|s| s.to_lowercase())
            .unwrap_or_default();
        match ext.as_str() {
            "pdf" => "application/pdf",
            "doc" | "docx" => {
                "application/vnd.openxmlformats-officedocument.wordprocessingml.document"
            }
            "xls" | "xlsx" => "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
            "ppt" | "pptx" => {
                "application/vnd.openxmlformats-officedocument.presentationml.presentation"
            }
            "txt" => "text/plain",
            "jpg" | "jpeg" => "image/jpeg",
            "png" => "image/png",
            _ => "application/octet-stream",
        }
    }

    async fn upload_file(&self, file_data: &[u8], file_name: &str) -> PreviewResult<String> {
        let mime = Self::detect_mime_type(file_name);
        let form = reqwest::multipart::Form::new().part(
            "file",
            reqwest::multipart::Part::bytes(file_data.to_vec())
                .file_name(file_name.to_string())
                .mime_str(mime)
                .map_err(|e| PreviewError::UploadFailed(e.to_string()))?,
        );

        let url = format!(
            "{}/FileUploader.ashx",
            self.doc_server_url.trim_end_matches('/')
        );
        let resp = self.client.post(&url).multipart(form).send().await?;

        if !resp.status().is_success() {
            return Err(PreviewError::UploadFailed(format!(
                "status: {}",
                resp.status()
            )));
        }

        let text = resp.text().await?;
        let file_url = text.trim().trim_matches('"').to_string();
        Ok(file_url)
    }

    async fn convert_file(&self, file_url: &str, target_format: &str) -> PreviewResult<String> {
        let payload = serde_json::json!({
            "url": file_url,
            "outputtype": target_format,
            "async": false,
        });

        let url = format!(
            "{}/ConvertService.ashx",
            self.doc_server_url.trim_end_matches('/')
        );
        let resp = self.client.post(&url).json(&payload).send().await?;

        if !resp.status().is_success() {
            return Err(PreviewError::ConversionFailed(format!(
                "status: {}",
                resp.status()
            )));
        }

        let result: serde_json::Value = resp.json().await?;
        let preview_url = result
            .get("fileUrl")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        if preview_url.is_empty() {
            return Err(PreviewError::ConversionFailed(
                "empty preview url".to_string(),
            ));
        }

        Ok(preview_url)
    }
}

#[async_trait]
impl PreviewService for OnlyOfficePreview {
    async fn preview(&self, file_data: &[u8], file_name: &str) -> PreviewResult<PreviewResponse> {
        let file_url = self.upload_file(file_data, file_name).await?;
        let target = "pdf".to_string();
        let preview_url = self.convert_file(&file_url, &target).await?;
        Ok(PreviewResponse {
            success: true,
            preview_url: Some(preview_url),
            download_url: Some(file_url.clone()),
            file_url: Some(file_url),
            message: "preview generated".to_string(),
        })
    }

    async fn convert(
        &self,
        file_data: &[u8],
        file_name: &str,
        target_format: &str,
    ) -> PreviewResult<Vec<u8>> {
        let file_url = self.upload_file(file_data, file_name).await?;
        let preview_url = self.convert_file(&file_url, target_format).await?;
        let resp = self.client.get(&preview_url).send().await?;
        if !resp.status().is_success() {
            return Err(PreviewError::DownloadFailed(format!(
                "status: {}",
                resp.status()
            )));
        }
        Ok(resp.bytes().await?.to_vec())
    }

    async fn upload(&self, file_data: &[u8], file_name: &str) -> PreviewResult<UploadedFile> {
        let file_url = self.upload_file(file_data, file_name).await?;
        Ok(UploadedFile {
            file_url,
            file_name: file_name.to_string(),
            file_type: OnlyOfficePreview::detect_mime_type(file_name).to_string(),
            size: file_data.len() as u64,
        })
    }

    async fn download(&self, file_url: &str) -> PreviewResult<Vec<u8>> {
        let resp = self.client.get(file_url).send().await?;
        if !resp.status().is_success() {
            return Err(PreviewError::DownloadFailed(format!(
                "status: {}",
                resp.status()
            )));
        }
        Ok(resp.bytes().await?.to_vec())
    }
}

#[derive(Clone)]
pub struct LibreOfficePreview {
    pub temp_dir: PathBuf,
}

impl Default for LibreOfficePreview {
    fn default() -> Self {
        Self {
            temp_dir: std::env::temp_dir().join("oa4rust_preview"),
        }
    }
}

impl LibreOfficePreview {
    pub fn new(temp_dir: impl Into<PathBuf>) -> Self {
        Self {
            temp_dir: temp_dir.into(),
        }
    }

    /// 文件名安全归一（2026-10 安全审计）：只取路径末段，剥离目录/`..` 成分，
    /// 防止 `../../etc/cron.d/x` 这类穿越把 `temp_dir.join(file_name)` 写到沙箱外。
    /// 空名、纯 `.`/`..`、含 NUL 一律返回 None。
    fn safe_base_name(file_name: &str) -> Option<String> {
        if file_name.is_empty() || file_name.contains('\0') {
            return None;
        }
        let base = std::path::Path::new(file_name)
            .file_name()
            .and_then(|s| s.to_str())?;
        if base.is_empty() || base == "." || base == ".." {
            return None;
        }
        Some(base.to_string())
    }

    fn detect_target_format(file_name: &str) -> &'static str {
        let ext = PathBuf::from(file_name)
            .extension()
            .and_then(|s| s.to_str())
            .map(|s| s.to_lowercase())
            .unwrap_or_default();
        match ext.as_str() {
            "pdf" => "pdf",
            "doc" | "docx" => "pdf",
            "xls" | "xlsx" => "pdf",
            "ppt" | "pptx" => "pdf",
            "txt" => "pdf",
            "odt" => "pdf",
            "ods" => "pdf",
            "odp" => "pdf",
            _ => "pdf",
        }
    }

    async fn convert_locally(
        &self,
        input_path: &std::path::Path,
        target_format: &str,
    ) -> PreviewResult<PathBuf> {
        let output_dir = self.temp_dir.join("output");
        tokio::fs::create_dir_all(&output_dir).await?;

        // 2026-10 安全审计：target_format 会被拼进输出文件名与 libreoffice
        // --convert-to 参数，限定纯字母数字，杜绝 ../ 穿越与畸形过滤器名。
        if !target_format
            .chars()
            .all(|c| c.is_ascii_alphanumeric())
            || target_format.is_empty()
        {
            return Err(PreviewError::ConversionFailed(
                "invalid target format".to_string(),
            ));
        }

        let output_path = output_dir.join(format!(
            "{}.{}",
            input_path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("output"),
            target_format
        ));

        let mut cmd = tokio::process::Command::new("libreoffice");
        cmd.arg("--headless")
            .arg("--convert-to")
            .arg(target_format)
            .arg("--outdir")
            .arg(&output_dir)
            .arg(input_path);

        info!(cmd = ?cmd, "running libreoffice conversion");
        let status = cmd
            .status()
            .await
            .map_err(|e| PreviewError::ConversionFailed(e.to_string()))?;
        if !status.success() {
            return Err(PreviewError::ConversionFailed(
                "libreoffice exited with error".to_string(),
            ));
        }

        Ok(output_path)
    }
}

#[async_trait]
impl PreviewService for LibreOfficePreview {
    async fn preview(&self, file_data: &[u8], file_name: &str) -> PreviewResult<PreviewResponse> {
        let target = Self::detect_target_format(file_name);
        let _converted = self.convert(file_data, file_name, target).await?;
        let output_path = self.temp_dir.join("output").join(format!(
            "{}.{}",
            Self::safe_base_name(file_name)
                .and_then(|b| std::path::Path::new(&b).file_stem().map(|s| s.to_string_lossy().into_owned()))
                .unwrap_or_else(|| "output".to_string()),
            target
        ));
        let file_url = format!("file://{}", output_path.display());
        Ok(PreviewResponse {
            success: true,
            preview_url: Some(file_url.clone()),
            download_url: Some(file_url.clone()),
            file_url: Some(file_url),
            message: "converted locally".to_string(),
        })
    }

    async fn convert(
        &self,
        file_data: &[u8],
        file_name: &str,
        target_format: &str,
    ) -> PreviewResult<Vec<u8>> {
        // 2026-10 安全审计：文件名校验，防路径穿越写到沙箱外
        let base = Self::safe_base_name(file_name)
            .ok_or_else(|| PreviewError::ConversionFailed("invalid file name".to_string()))?;
        tokio::fs::create_dir_all(&self.temp_dir).await?;
        let input_path = self.temp_dir.join(&base);
        tokio::fs::write(&input_path, file_data).await?;

        let output_path = self.convert_locally(&input_path, target_format).await?;
        let result = tokio::fs::read(&output_path).await?;
        Ok(result)
    }

    async fn upload(&self, file_data: &[u8], file_name: &str) -> PreviewResult<UploadedFile> {
        // 2026-10 安全审计：文件名校验，防路径穿越写到沙箱外
        let base = Self::safe_base_name(file_name)
            .ok_or_else(|| PreviewError::UploadFailed("invalid file name".to_string()))?;
        tokio::fs::create_dir_all(&self.temp_dir).await?;
        let file_url = format!("file://{}/{}", self.temp_dir.display(), base);
        tokio::fs::write(self.temp_dir.join(&base), file_data).await?;
        Ok(UploadedFile {
            file_url,
            file_name: base.clone(),
            file_type: "application/octet-stream".to_string(),
            size: file_data.len() as u64,
        })
    }

    async fn download(&self, file_url: &str) -> PreviewResult<Vec<u8>> {
        let path = file_url.trim_start_matches("file://");
        if path.is_empty() {
            return Err(PreviewError::DownloadFailed("invalid file url".to_string()));
        }
        // 2026-10 安全审计：只允许读取沙箱（temp_dir）内的文件，防 file:// 穿越读任意文件
        let p = std::path::Path::new(path);
        if !p.starts_with(&self.temp_dir) {
            return Err(PreviewError::DownloadFailed(
                "file url escapes preview sandbox".to_string(),
            ));
        }
        for seg in p.components() {
            if matches!(seg, std::path::Component::ParentDir) {
                return Err(PreviewError::DownloadFailed(
                    "file url escapes preview sandbox".to_string(),
                ));
            }
        }
        Ok(std::fs::read(p)?)
    }
}

pub async fn preview_upload_handler(
    State(service): State<Arc<dyn PreviewService>>,
    Json(req): Json<PreviewRequest>,
) -> Json<PreviewResponse> {
    let file_data = match general_purpose::STANDARD.decode(&req.file_data) {
        Ok(data) => data,
        Err(_) => {
            return Json(PreviewResponse {
                success: false,
                preview_url: None,
                download_url: None,
                file_url: None,
                message: "invalid base64 file data".to_string(),
            });
        }
    };
    match service.preview(&file_data, &req.file_name).await {
        Ok(resp) => Json(resp),
        Err(e) => Json(PreviewResponse {
            success: false,
            preview_url: None,
            download_url: None,
            file_url: None,
            message: e.to_string(),
        }),
    }
}

pub async fn preview_convert_handler(
    State(service): State<Arc<dyn PreviewService>>,
    Json(req): Json<PreviewRequest>,
) -> Json<PreviewResponse> {
    let file_data = match general_purpose::STANDARD.decode(&req.file_data) {
        Ok(data) => data,
        Err(_) => {
            return Json(PreviewResponse {
                success: false,
                preview_url: None,
                download_url: None,
                file_url: None,
                message: "invalid base64 file data".to_string(),
            });
        }
    };
    let target = req.target_format.unwrap_or_else(|| "pdf".to_string());
    match service.convert(&file_data, &req.file_name, &target).await {
        Ok(data) => {
            let encoded = general_purpose::STANDARD.encode(data);
            Json(PreviewResponse {
                success: true,
                preview_url: None,
                download_url: Some(format!("data:application/octet-stream;base64,{}", encoded)),
                file_url: None,
                message: format!("converted to {}", target),
            })
        }
        Err(e) => Json(PreviewResponse {
            success: false,
            preview_url: None,
            download_url: None,
            file_url: None,
            message: e.to_string(),
        }),
    }
}

pub fn preview_route<S: PreviewService + 'static>(service: S) -> Router {
    Router::new()
        .route(
            "/preview/upload",
            axum::routing::post(preview_upload_handler),
        )
        .route(
            "/preview/convert",
            axum::routing::post(preview_convert_handler),
        )
        .with_state(Arc::new(service))
}

#[cfg(test)]
#[allow(clippy::module_inception)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_preview_upload_route() {
        let service = LibreOfficePreview::default();
        let app = preview_route(service);
        let req = PreviewRequest {
            file_name: "test.pdf".to_string(),
            file_data: general_purpose::STANDARD.encode(b"hello pdf"),
            file_type: Some("application/pdf".to_string()),
            target_format: None,
        };
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/preview/upload")
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
    async fn test_preview_convert_route() {
        let service = LibreOfficePreview::default();
        let app = preview_route(service);
        let req = PreviewRequest {
            file_name: "test.txt".to_string(),
            file_data: general_purpose::STANDARD.encode(b"hello"),
            file_type: Some("text/plain".to_string()),
            target_format: Some("pdf".to_string()),
        };
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/preview/convert")
                    .method("POST")
                    .header("content-type", "application/json")
                    .body(Body::from(serde_json::to_vec(&req).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[test]
    fn test_detect_mime_type() {
        assert_eq!(
            OnlyOfficePreview::detect_mime_type("test.pdf"),
            "application/pdf"
        );
        assert_eq!(
            OnlyOfficePreview::detect_mime_type("test.docx"),
            "application/vnd.openxmlformats-officedocument.wordprocessingml.document"
        );
        assert_eq!(
            OnlyOfficePreview::detect_mime_type("test.txt"),
            "text/plain"
        );
        assert_eq!(
            OnlyOfficePreview::detect_mime_type("test.unknown"),
            "application/octet-stream"
        );
    }

    #[test]
    fn test_detect_target_format() {
        assert_eq!(LibreOfficePreview::detect_target_format("test.pdf"), "pdf");
        assert_eq!(LibreOfficePreview::detect_target_format("test.docx"), "pdf");
        assert_eq!(LibreOfficePreview::detect_target_format("test.xlsx"), "pdf");
    }
}

#[cfg(test)]
mod tests_generated;

#[cfg(test)]
mod security_preview_tests {
    use super::*;

    /// 2026-10 安全审计：路径穿越必须被 safe_base_name 拦截。
    #[test]
    fn safe_base_name_blocks_traversal() {
        assert_eq!(LibreOfficePreview::safe_base_name("a.docx").as_deref(), Some("a.docx"));
        assert_eq!(
            LibreOfficePreview::safe_base_name("../../etc/cron.d/x").as_deref(),
            Some("x"),
            "目录段必须被剥离，只保留末段"
        );
        assert_eq!(
            LibreOfficePreview::safe_base_name(r"..\..\windows\system.ini").as_deref(),
            Some("system.ini"),
            "Windows 反斜杠穿越同样剥离"
        );
        assert_eq!(LibreOfficePreview::safe_base_name(".."), None);
        assert_eq!(LibreOfficePreview::safe_base_name("."), None);
        assert_eq!(LibreOfficePreview::safe_base_name(""), None);
        assert_eq!(LibreOfficePreview::safe_base_name("bad\0name.doc"), None);
    }
}
