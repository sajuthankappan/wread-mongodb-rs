pub mod crud_repository;
pub mod data_error;
pub mod unexpected_error;

#[cfg(feature = "axum-handlers")]
pub mod axum_error_mapper;

pub use mongodb;
