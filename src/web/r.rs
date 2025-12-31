use axum::Json;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::Serialize;

use crate::web::error::WebError;
use crate::web::code::Code;
use rust_i18n::t;
use tracing::{debug, error};
use validator::ValidationErrors;

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
        let status = StatusCode::from_u16(self.code as u16).unwrap_or(StatusCode::OK);
        (status, Json(self)).into_response()
    }
}

fn map_err(err: WebError) -> (i32, String) {
    match err {
        WebError::Val(err) => {
            debug!("{:?}", err);
            let msg = format_validation_errors(&err);
            (Code::IllegalParam.as_i32(), msg)
        }
        WebError::Biz(code, args) => (code, translate(code, &args)),
        _ => {
            error!("{:?}", err);
            (
                Code::InternalServerError.as_i32(),
                translate(Code::InternalServerError.as_i32(), &vec![]),
            )
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
    // 使用 t! 宏进行翻译，如果有参数则进行替换
    // 注意：rust-i18n 的 t! 宏在运行时替换可能需要不同的方式，这里保持手动替换逻辑作为后备或增强
    // 但更标准的做法是尽可能利用 t! 的插值能力。
    // 由于 t! 宏是编译时展开，这里用 t!(&key) 可能只能拿到原始模板字符串，
    // 所以手动 replace 是正确的运行时处理动态 key 的方式。
    let mut message = t!(&key).to_string();
    for (k, v) in params {
        // 尝试替换 {key} 格式的占位符
        message = message.replace(&format!("{{{}}}", k), v);
        // 尝试替换 %{key} 格式的占位符（有些 i18n 库习惯）
        message = message.replace(&format!("%{{{}}}", k), v);
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
