use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    headers,
    response::IntoResponse,
    TypedHeader,
};

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

async fn handle_socket(mut socket: WebSocket, _who: SocketAddr) {
    if let Some(msg) = socket.recv().await {
        if let Ok(msg) = msg {
            println!("Client says: {:?}", msg);
            //客户端发什么，服务端就回什么（只是演示而已）
            if socket
                .send(Message::Text(format!("{:?}", msg)))
                .await
                .is_err()
            {
                println!("client disconnected");
            }
        } else {
            println!("client disconnected");
        }
    }
}
