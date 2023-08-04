use actix_web::{http::StatusCode, HttpResponse};
use bcrypt::BcryptError;
use diesel::r2d2::{Error as R2D2Error, PoolError};
use diesel::result::{DatabaseErrorKind, Error as DieselError};
use serde_json::json;
use serde_json::Value as JsonValue;
use std::convert::From;
use serde::Serialize;
use thiserror::Error;
use uuid::Error as UuidError;

#[derive(Error, Debug, Serialize)]
pub enum AppError {
    // 401
    #[error("Unauthorized: {}", _0)]
    Unauthorized(JsonValue),

    // 403
    #[error("Forbidden: {}", _0)]
    Forbidden(JsonValue),

    // 404
    #[error("Not Found: {}", _0)]
    NotFound(JsonValue),

    // 409
    #[error("Exists: {}", _0)]
    Exists(JsonValue),

    // 422
    #[error("Unprocessable Entity: {}", _0)]
    UnprocessableEntity(JsonValue),

    // 500
    #[error("Internal Server Error")]
    InternalServerError,
}

impl actix_web::error::ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match *self {
            AppError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            AppError::Forbidden(_) => StatusCode::FORBIDDEN,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::Exists(_) => StatusCode::CONFLICT,
            AppError::UnprocessableEntity(_) => StatusCode::UNPROCESSABLE_ENTITY,
            AppError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::Unauthorized(ref msg) => HttpResponse::Unauthorized().json(msg),
            AppError::Forbidden(ref msg) => HttpResponse::Forbidden().json(msg),
            AppError::NotFound(ref msg) => HttpResponse::NotFound().json(msg),
            AppError::Exists(ref msg) => HttpResponse::Conflict().json(msg),
            AppError::UnprocessableEntity(ref msg) => HttpResponse::UnprocessableEntity().json(msg),
            AppError::InternalServerError => {
                HttpResponse::InternalServerError().json("Internal Server Error")
            }
        }
    }
}

impl From<PoolError> for AppError {
    fn from(_err: PoolError) -> Self {
        AppError::InternalServerError
    }
}

impl From<BcryptError> for AppError {
    fn from(_err: BcryptError) -> Self {
        AppError::InternalServerError
    }
}

impl From<R2D2Error> for AppError {
    fn from(_err: R2D2Error) -> Self {
        AppError::InternalServerError
    }
}

impl From<DieselError> for AppError {
    fn from(err: DieselError) -> Self {
        match err {
            DieselError::DatabaseError(kind, info) => {
                if let DatabaseErrorKind::UniqueViolation = kind {
                    let message = info.details().unwrap_or_else(|| info.message()).to_string();
                    if message.ends_with("already exists.") {
                        AppError::Exists(json!({"error": message}))
                    } else {
                        AppError::UnprocessableEntity(json!({ "error": message }))
                    }
                } else {
                    AppError::InternalServerError
                }
            }
            DieselError::NotFound => {
                AppError::NotFound(json!({ "error": "requested record was not found" }))
            }
            _ => AppError::InternalServerError,
        }
    }
}

impl From<UuidError> for AppError {
    fn from(_err: UuidError) -> Self {
        AppError::NotFound(json!({"error":"Uuid is invalid."}))
    }
}