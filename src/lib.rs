use std::sync::{Arc, RwLock};

pub mod camera_opencv;
pub mod send_frame;

pub type SharedState = Arc<RwLock<AppState>>;

#[derive(Debug, Default)]
pub struct AppState {
    pub frame: Vec<u8>,
}
