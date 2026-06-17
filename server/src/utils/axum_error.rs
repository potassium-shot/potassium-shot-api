use axum::Json;

pub type AxumResult<T> = Result<T, AxumError>;

pub struct AxumError(Json<AxumErrorStub>);

impl axum::response::IntoResponse for AxumError {
    fn into_response(self) -> axum::response::Response {
        self.0.into_response()
    }
}

#[derive(serde::Serialize)]
struct AxumErrorStub {
    error: String,
}

impl From<anyhow::Error> for AxumError {
    fn from(value: anyhow::Error) -> Self {
        Self(Json(AxumErrorStub {
            error: value.to_string(),
        }))
    }
}
