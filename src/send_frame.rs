use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    headers,
    response::IntoResponse,
    TypedHeader,
};
use std::io::Cursor;
use std::net::SocketAddr;
// 允许提取连接用户的IP
use axum::extract::connect_info::ConnectInfo;

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        String::from("Unknown browser")
    };
    tracing::debug!("`{user_agent:?}` at {addr:?} connected");
    ws.on_upgrade(move |socket| handle_socket(socket, addr))
}

async fn handle_socket(mut socket: WebSocket, addr: SocketAddr) {
    tracing::debug!("`{addr:?}`disconnected");
}
