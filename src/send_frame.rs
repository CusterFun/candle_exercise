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
use nokhwa::pixel_format::{RgbAFormat, RgbFormat};
use nokhwa::utils::{ApiBackend, RequestedFormat, RequestedFormatType};
use nokhwa::{nokhwa_initialize, query, CallbackCamera};

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
    // only needs to be run on OSX
    nokhwa_initialize(|granted| {
        tracing::debug!("User said {}", granted);
    });
    let cameras = query(ApiBackend::Auto).unwrap();
    cameras.iter().for_each(|cam| tracing::debug!("{:?}", cam));

    let format = RequestedFormat::new::<RgbFormat>(RequestedFormatType::AbsoluteHighestFrameRate);

    let first_camera = cameras.last().unwrap(); // CameraIndex::Index(0);

    let mut threaded = CallbackCamera::new(first_camera.index().clone(), format, |_| {}).unwrap();
    threaded.open_stream().unwrap();

    #[allow(clippy::empty_loop)] // keep it running
    loop {
        let frame = threaded.poll_frame().unwrap();
        let image = frame.decode_image::<RgbAFormat>().unwrap();

        tracing::debug!(
            "{}x{} {} naripoggers",
            image.width(),
            image.height(),
            image.len()
        );

        //将图像转换为 PNG 字节
        let mut buf = vec![];
        let mut cursor = Cursor::new(&mut buf);
        image::DynamicImage::ImageRgba8(image)
            .write_to(&mut cursor, image::ImageOutputFormat::Png)
            .unwrap();

        // 通过 WebSocket 发送图像字节
        if let Err(e) = socket.send(Message::Binary(buf)).await {
            tracing::error!("Error sending frame: {:?}", e);
            break;
        }
    }

    tracing::debug!("`{addr:?}`disconnected");
}
