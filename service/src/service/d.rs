use axum::extract::Path;
use axum::http::header;
use axum::response::IntoResponse;
use rust_embed_for_web::{EmbedableFile, RustEmbed};

#[derive(RustEmbed)]
#[folder = "../dashboard/out"]
struct DashboardAssets;

pub async fn get(path: Option<Path<String>>) -> impl IntoResponse {
    if let Some(Path(path)) = path {
        let resource = DashboardAssets::get(&path);
        if let Some(resource) = resource {
            let content_type = if path.ends_with(".js") {
                "text/javascript"
            } else if path.ends_with(".css") {
                "text/css"
            } else if path.ends_with(".png") {
                "image/png"
            } else if path.ends_with(".txt") {
                "text/plain"
            } else {
                "text/html"
            };
            return ([(header::CONTENT_TYPE, content_type)], resource.data());
        }

        if let Some(resource) = DashboardAssets::get(&format!("{}index.html", &path)) {
            return ([(header::CONTENT_TYPE, "text/html")], resource.data());
        }
    }

    (
        [(header::CONTENT_TYPE, "text/html")],
        DashboardAssets::get("index.html").unwrap().data(),
    )
}
