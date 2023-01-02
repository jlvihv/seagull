use axum::{http::StatusCode, response::IntoResponse, Json};

pub struct User {
    pub username: String,
    pub password: String,
}

pub async fn register() {}
pub async fn login(Json(payload): Json<User>) -> impl IntoResponse {
    if payload.username == "admin" && payload.password == "admin" {
        (StatusCode::OK, "ok")
    } else {
        (StatusCode::UNAUTHORIZED, "unauthorized")
    }
}
