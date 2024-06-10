use axum::{
    http::{header::CONTENT_TYPE, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};

use serde::Serialize;
use thiserror::Error;

#[derive(Serialize)]
pub struct ProblemDetails {
    #[serde(rename = "type")]
    problem_type: String,
    title: String,
    status: u16,
    detail: String,
    instance: String,
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status_code: StatusCode = match &self {
            AppError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
        };

        let problem_details = ProblemDetails {
            problem_type: "https://example.com/probs/internal-server-error".to_string(),
            title: self.to_string(),
            status: status_code.as_u16(),
            detail: "An unexpected error occurred".to_string(),
            instance: "/".to_string(), // In a real application, provide the request path
        };

        let body = serde_json::to_string(&problem_details).unwrap();

        (
            status_code,
            [(CONTENT_TYPE, "application/problem+json")],
            body,
        )
            .into_response()
    }
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Internal Server Error")]
    InternalServerError(#[from] anyhow::Error),

    #[error("Bad Request")]
    BadRequest(String),
}

async fn handler() -> anyhow::Result<String, AppError> {
    let result: anyhow::Result<()> = Err(anyhow::anyhow!("Something went wrong"));
    result?;
    Ok("Hello, World!".to_string())
}
