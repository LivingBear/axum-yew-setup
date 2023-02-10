use axum::response::IntoResponse;

pub async fn hi_route() -> impl IntoResponse {
    "hi from server!"
}