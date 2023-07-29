//! Session IDs are calculated from the client's IP address, and the current day.

use std::net::SocketAddr;

use base64::Engine;
use chrono::{DateTime, Days, Utc};
use getrandom::getrandom;
use http::HeaderMap;
use lazy_static::lazy_static;
use sha2::{Digest, Sha256};
use tokio::sync::RwLock;

struct TodayCode {
    last_updated: DateTime<Utc>,
    code: String,
}

impl TodayCode {
    fn new() -> Self {
        let mut bytes = [0u8; 32];
        getrandom(&mut bytes).unwrap();
        let code = base64::engine::general_purpose::STANDARD_NO_PAD.encode(bytes);
        Self {
            last_updated: Utc::now(),
            code,
        }
    }
}

lazy_static! {
    static ref TODAY_CODE: RwLock<TodayCode> = RwLock::new(TodayCode::new());
}

/// Gets the today code, updating it if required.
async fn get_today_code() -> String {
    let today_code = TODAY_CODE.read().await;
    // If lat_updated + 1 is less than now, it has been more than a day since last update
    if today_code
        .last_updated
        .checked_add_days(Days::new(1))
        .unwrap()
        < Utc::now()
    {
        drop(today_code);
        let mut today_code = TODAY_CODE.write().await;
        if today_code
            .last_updated
            .checked_add_days(Days::new(1))
            .unwrap()
            < Utc::now()
        {
            // Double check, because someone else could have grabbed the lock while we were waiting and updated it
            *today_code = TodayCode::new();
        }
        return today_code.code.clone();
    }
    today_code.code.clone()
}

pub struct SessionId(pub String);

pub async fn get_session_id(
    info: &SocketAddr,
    headers: &HeaderMap,
    forwarded_ip_header: &Option<String>,
) -> SessionId {
    let today = Utc::now().format("%Y-%m-%d").to_string();
    let ip = if let Some(forwarded_ip_header) = forwarded_ip_header {
        headers
            .get(forwarded_ip_header)
            .and_then(|v| v.to_str().ok())
            .map(|v| v.to_string())
            .unwrap_or_else(|| {
                // TODO log error that forwarded ip header was set up, but not found in request
                info.ip().to_string()
            })
    } else {
        info.ip().to_string()
    };

    let user_agent = headers
        .get("User-Agent")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    let mut hasher = Sha256::new();
    hasher.update("basalytics");
    hasher.update(today);
    hasher.update(get_today_code().await);
    hasher.update(ip);
    hasher.update(user_agent);

    let hash = hasher.finalize();

    SessionId(base64::engine::general_purpose::STANDARD_NO_PAD.encode(hash))
}
