use axum::Json;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::Serialize;

use validator::ValidationErrors;
use rivus_core::code::Code;
use rust_i18n::t;
use crate::web::error::WebError;
use tracing::{debug, error};

#[derive(Serialize)]
pub struct R<T: Serialize> {
    pub code: i32,
    pub message: String,
    pub data: Option<T>,
}

impl<T: Serialize> R<T> {
    pub fn ok(data: T) -> Self {
        let code = Code::Ok.as_i32();
        Self {
            code,
            message: translate(code, &vec![]),
            data: Some(data),
        }
    }

    pub fn err(err: WebError) -> Self {
        let (code, message) = map_err(err);
        Self {
            code,
            message,
            data: None,
        }
    }

    pub fn from<E>(result: Result<T, E>) -> Self
    where
        WebError: From<E>,
    {
        match result {
            Ok(data) => Self::ok(data),
            Err(err) => Self::err(WebError::from(err)),
        }
    }
}


impl<T: Serialize> From<T> for R<T> {
    fn from(data: T) -> Self {
        Self::ok(data)
    }
}

impl<T: Serialize> From<WebError> for R<T> {
    fn from(err: WebError) -> Self {
        Self::err(err)
    }
}

impl R<()> {
    pub fn void() -> Self {
        Self::ok(())
    }

    pub fn from_unit<E>(result: Result<(), E>) -> Self
    where
        WebError: From<E>,
    {
        match result {
            Ok(_) => Self::ok(()),
            Err(err) => Self::err(WebError::from(err)),
        }
    }
}

impl<T: Serialize> IntoResponse for R<T> {
    fn into_response(self) -> axum::response::Response {
        let status = if self.code == Code::InternalServerError.as_i32() {
            StatusCode::INTERNAL_SERVER_ERROR
        } else {
            StatusCode::OK
        };

        (status, Json(self)).into_response()
    }
}


fn map_err(err: WebError) -> (i32, String) {
    match err {
        WebError::DbError(err) => {
            error!("{:?}", err);
            (
                Code::InternalServerError.as_i32(),
                translate(Code::InternalServerError.as_i32(), &vec![]),
            )
        }
        WebError::Error(e) => {
            error!("{:?}", e);
            (e.code, translate(e.code, &e.args))
        }
        WebError::Val(err) => {
            debug!("{:?}", err);
            let msg = format_validation_errors(&err);
            (Code::IllegalParam.as_i32(), msg)
        }
        WebError::Io(e) => {
            error!("{:?}", e);
            (
                Code::InternalServerError.as_i32(),
                translate(Code::InternalServerError.as_i32(), &vec![]),
            )
        }
        WebError::System(e) => {
            error!("{:?}", e);
            (Code::InternalServerError.as_i32(), e)
        }
    }
}

fn format_validation_errors(err: &ValidationErrors) -> String {
    let mut msgs = Vec::new();
    for (field, errs) in err.field_errors() {
        for e in errs {
            let detail = match e.code.as_ref() {
                "required" => "is required".to_string(),
                "length" => {
                    let min = e.params.get("min");
                    let max = e.params.get("max");
                    match (min, max) {
                        (Some(min), Some(max)) => {
                            format!("length must be between {} and {}", min, max)
                        }
                        (Some(min), None) => format!("length must be at least {}", min),
                        (None, Some(max)) => format!("length must be at most {}", max),
                        _ => "length is invalid".to_string(),
                    }
                }
                "range" => {
                    let min = e.params.get("min");
                    let max = e.params.get("max");
                    match (min, max) {
                        (Some(min), Some(max)) => {
                            format!("must be between {} and {}", min, max)
                        }
                        (Some(min), None) => format!("must be at least {}", min),
                        (None, Some(max)) => format!("must be at most {}", max),
                        _ => "value is out of range".to_string(),
                    }
                }
                "email" => "must be a valid email".to_string(),
                _ => e
                    .message
                    .clone()
                    .map(|m| m.to_string())
                    .unwrap_or_else(|| format!("invalid ({})", e.code)),
            };
            msgs.push(format!("{}: {}", field, detail));
        }
    }
    if msgs.is_empty() {
        translate(Code::IllegalParam.as_i32(), &vec![])
    } else {
        msgs.join("; ")
    }
}

fn translate(code: i32, params: &Vec<(String, String)>) -> String {
    let key = code.to_string();
    let mut message = t!(&key).to_string();
    for (k, v) in params {
        message = message.replace(&format!("{{{}}}", k), v);
    }
    message
}

#[macro_export]
macro_rules! r {
    ($result:expr) => {
        match $result {
            Ok(value) => value,
            Err(err) => return $crate::web::r::R::err(err.into()),
        }
    };
}
