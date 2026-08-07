pub mod avatar;
pub mod password;
pub mod personal;
pub mod routes;

pub use routes::personal_extend_router;

#[cfg(test)]
mod tests;
