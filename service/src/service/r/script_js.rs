use axum::http::header;
use axum::response::IntoResponse;

pub async fn get() -> impl IntoResponse {
    // Respond with an 1x1 transparent png
    (
        [(header::CONTENT_TYPE, "text/javascript")],
        include_bytes!("../../../../tracker/build/out.js"),
    )
}
