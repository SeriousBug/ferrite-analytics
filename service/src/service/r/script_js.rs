use axum::http::header;
use axum::response::IntoResponse;

pub async fn get() -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "text/javascript")],
        include_bytes!("../../../../tracker/build/out.js"),
    )
}
