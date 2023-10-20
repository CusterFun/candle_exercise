use opencv::videoio::VideoCapture;
use std::sync::{Arc, Mutex, RwLock};

pub type SharedState = Arc<RwLock<AppState>>;
#[derive(Default)]
pub struct AppState {
    pub camera: Option<Arc<Mutex<VideoCapture>>>,
}
