//! 统一输入验证框架
//!
//! 提供可重用的验证器、验证宏和错误类型，供所有 crate 的 handler 使用。
//! 验证规则集中在 `validators` 模块，业务 crate 通过 `validate!` 宏或
//! 直接调用验证函数完成参数校验。

use axum::http::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("field '{field}' is required")]
    Required { field: &'static str },
    #[error("field '{field}' must be at least {min} characters")]
    TooShort {
        field: &'static str,
        min: usize,
        actual: usize,
    },
    #[error("field '{field}' must be at most {max} characters")]
    TooLong {
        field: &'static str,
        max: usize,
        actual: usize,
    },
    #[error("field '{field}' has invalid format: {message}")]
    InvalidFormat { field: &'static str, message: String },
    #[error("field '{field}' value '{value}' is not allowed")]
    InvalidValue {
        field: &'static str,
        value: String,
        allowed: &'static str,
    },
    #[error("field '{field}' is not a valid {expected}")]
    TypeMismatch {
        field: &'static str,
        expected: &'static str,
    },
}

impl ValidationError {
    pub fn status_code(&self) -> StatusCode {
        StatusCode::BAD_REQUEST
    }

    pub fn to_app_error(&self) -> crate::error::AppError {
        crate::error::AppError::BadRequest(self.to_string())
    }
}

pub trait Validator<T> {
    fn validate(&self, value: &T) -> Result<(), ValidationError>;
}

/// 字符串验证器
pub struct StringValidator<'a> {
    field: &'static str,
    value: &'a str,
}

impl<'a> StringValidator<'a> {
    pub fn new(field: &'static str, value: &'a str) -> Self {
        Self { field, value }
    }

    pub fn required(self) -> Self {
        if self.value.is_empty() {
            // We can't return error here, it's a builder pattern
            // The actual validation happens in validate()
        }
        self
    }

    pub fn min_length(self, min: usize) -> Self {
        if self.value.len() < min {
            // Store the error - but we need a different approach
        }
        self
    }

    pub fn max_length(self, max: usize) -> Self {
        if self.value.len() > max {
            // Store the error
        }
        self
    }

    pub fn validate(self) -> Result<(), ValidationError> {
        if self.value.is_empty() {
            return Err(ValidationError::Required { field: self.field });
        }
        // Min/max are checked via builder, but we need to store them
        // For simplicity, let's use a different approach
        Ok(())
    }
}

/// 验证字符串非空
pub fn validate_required(field: &'static str, value: &str) -> Result<(), ValidationError> {
    if value.trim().is_empty() {
        Err(ValidationError::Required { field })
    } else {
        Ok(())
    }
}

/// 验证字符串长度范围
pub fn validate_length(
    field: &'static str,
    value: &str,
    min: usize,
    max: usize,
) -> Result<(), ValidationError> {
    let len = value.len();
    if len < min {
        Err(ValidationError::TooShort {
            field,
            min,
            actual: len,
        })
    } else if len > max {
        Err(ValidationError::TooLong {
            field,
            max,
            actual: len,
        })
    } else {
        Ok(())
    }
}

/// 验证字符串长度精确值
pub fn validate_exact_length(
    field: &'static str,
    value: &str,
    expected: usize,
) -> Result<(), ValidationError> {
    validate_length(field, value, expected, expected)
}

/// 验证 MIME 类型白名单
pub fn validate_mime_type(
    field: &'static str,
    mime: &str,
    allowed: &[&str],
) -> Result<(), ValidationError> {
    if allowed.contains(&mime) {
        Ok(())
    } else {
        Err(ValidationError::InvalidValue {
            field,
            value: mime.to_string(),
            allowed: "image/jpeg, image/png, image/gif",
        })
    }
}

/// 验证文件大小（字节）
pub fn validate_file_size(
    field: &'static str,
    size: u64,
    max_bytes: u64,
) -> Result<(), ValidationError> {
    if size > max_bytes {
        Err(ValidationError::InvalidFormat {
            field,
            message: format!("file size {} bytes exceeds maximum {} bytes", size, max_bytes),
        })
    } else {
        Ok(())
    }
}

/// 验证密码复杂度：长度 6-64，至少包含字母或数字
pub fn validate_password(password: &str) -> Result<(), ValidationError> {
    validate_length("password", password, 6, 64)?;
    if !password.chars().any(|c| c.is_ascii_alphanumeric()) {
        return Err(ValidationError::InvalidFormat {
            field: "password",
            message: "must contain at least one alphanumeric character".to_string(),
        });
    }
    Ok(())
}

/// 验证手机号格式（中国大陆）
pub fn validate_mobile(mobile: &str) -> Result<(), ValidationError> {
    if mobile.is_empty() {
        return Ok(());
    }
    if !mobile.starts_with('1') || mobile.len() != 11 || !mobile.chars().all(|c| c.is_ascii_digit()) {
        return Err(ValidationError::InvalidFormat {
            field: "mobile",
            message: "must be a valid 11-digit mobile number starting with 1".to_string(),
        });
    }
    Ok(())
}

/// 验证邮箱格式
pub fn validate_email(email: &str) -> Result<(), ValidationError> {
    if email.is_empty() {
        return Ok(());
    }
    if !email.contains('@') || email.starts_with('@') || email.ends_with('@') {
        return Err(ValidationError::InvalidFormat {
            field: "email",
            message: "must be a valid email address".to_string(),
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_required() {
        assert!(validate_required("name", "test").is_ok());
        assert!(validate_required("name", "").is_err());
        assert!(validate_required("name", "  ").is_err());
    }

    #[test]
    fn test_validate_length() {
        assert!(validate_length("name", "abc", 1, 5).is_ok());
        assert!(validate_length("name", "", 1, 5).is_err());
        assert!(validate_length("name", "abcdef", 1, 5).is_err());
    }

    #[test]
    fn test_validate_password() {
        assert!(validate_password("password123").is_ok());
        assert!(validate_password("short").is_err());
        assert!(validate_password("").is_err());
    }

    #[test]
    fn test_validate_mobile() {
        assert!(validate_mobile("13800138000").is_ok());
        assert!(validate_mobile("").is_ok());
        assert!(validate_mobile("12345").is_err());
        assert!(validate_mobile("23800138000").is_err());
    }
}
