use axum::extract::{ConnectInfo, FromRequestParts};
use http::{request::Parts, StatusCode};
use std::{net::SocketAddr, sync::Arc};

use crate::{
    helpers::session_id::{get_session_id, SessionId},
    state,
};

#[async_trait::async_trait]
impl FromRequestParts<Arc<state::AppStateData>> for SessionId {
    type Rejection = StatusCode;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<state::AppStateData>,
    ) -> Result<Self, Self::Rejection> {
        let headers = &parts.headers;
        let ConnectInfo(info) = parts.extensions.get::<ConnectInfo<SocketAddr>>().unwrap();

        let session_id = get_session_id(info, headers, &state.forwarded_ip_header).await;

        Ok(session_id)
    }
}
