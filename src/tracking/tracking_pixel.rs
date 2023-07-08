use std::collections::HashMap;

use crate::state::AppState;
use crate::tracking::helpers::EventHelper;

use axum::http::{header, HeaderMap};
use axum::response::IntoResponse;

use super::helpers::UserAgent;

pub async fn tracking_pixel(headers: HeaderMap, state: AppState) -> impl IntoResponse {
    let user_agent_str = headers.get("user-agent").map(|v| v.to_str().unwrap());
    let user_agent = user_agent_str.map(|v| UserAgent::new(v));

    let mut properties: HashMap<&str, String> = HashMap::new();
    if let Some(user_agent) = user_agent {
        if let Some(platform) = user_agent.platform {
            properties.insert("platform", platform);
        }
        if let Some(browser) = user_agent.browser {
            properties.insert("browser", browser);
        }
    }

    state.save_event("tracking-pixel", properties).await;

    // Respond with an 1x1 transparent png
    (
        [
            // SVG content type
            (header::CONTENT_TYPE, "image/png"),
            // Do not cache this, we want to track every request
            (header::CACHE_CONTROL, "no-store"),
        ],
        // As small as an SVG image gets
        include_bytes!("../../resources/tracking_pixel.png"),
    )
}
