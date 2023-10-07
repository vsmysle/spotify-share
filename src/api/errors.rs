use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Json;

pub enum AppError {
    InternalServerError(anyhow::Error),
    NotFound(String),
}

fn handle_error(err: anyhow::Error) -> AppError {
    AppError::InternalServerError(err)
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        AppError::InternalServerError(err.into())
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            AppError::InternalServerError(inner) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Something went wrong! Error: {}", inner),
            ),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
        };

        let body = Json(serde_json::json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}
