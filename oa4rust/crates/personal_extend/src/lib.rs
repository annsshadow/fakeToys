pub mod avatar;
pub mod personal;
pub mod routes;

pub use routes::personal_extend_router;

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_generated;

#[cfg(test)]
mod password {
    #[derive(Debug, PartialEq, serde::Deserialize)]
    pub struct ChangePasswordRequest {
        pub old_password: String,
        pub new_password: String,
    }
    #[derive(Debug, PartialEq, serde::Deserialize)]
    pub struct ResetPasswordRequest {
        pub credential: String,
        pub code: String,
        pub password: String,
    }
    #[derive(Debug, PartialEq, serde::Deserialize)]
    pub struct VerifyPasswordRequest {
        pub credential: String,
        pub password: String,
    }
}

